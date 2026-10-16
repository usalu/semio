"""Mounts hub test files as in-crate tests of the module whose private items they name.
usage: mk_incrate.py plan | apply   (specs below)"""
import sys
from build import *
import manifest

APPLY = "apply" in sys.argv
HUB3 = R + "/📥️mounts/🧵️wp3-frame-core/🦀️.rs"
# (crate suffix, module file, test file, child module name, hub module name, cfg line)
SPECS = [
    ("host-winit", T + "/🪟️winit-app/🦀️.rs", E + "/🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs", "p3c_tests", "p3c_tests_host", "#[cfg(test)]"),
    ("host-winit", T + "/🪟️winit-app/🦀️.rs", E + "/🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs", "callback_latency_tests", "callback_latency_tests_host", "#[cfg(test)]"),
    ("host-winit", T + "/🪟️winit-app/🦀️.rs", E + "/🧪️tests/🔢️frame-generation-hold/🦀️.rs", "frame_generation_hold_tests", "frame_generation_hold_tests_host", "#[cfg(test)]"),
    ("frame-job", T + "/🧵️frame-job/🦀️.rs", E + "/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs", "frame_job_unit_tests", "tests_host", "#[cfg(test)]"),
    ("frame-job", T + "/🧵️frame-job/🦀️.rs", T + "/🧵️frame-job/📬️outcome/🧪️tests/🦀️.rs", "original_outcome_tests", "original_outcome_tests_host", "#[cfg(test)]"),
]


def module_names(path):
    text = read(path)
    names = {i["name"] for i in itm.table(text) if i["kind"] not in ("impl",)}
    names |= {l for c, v, p, l, g in uses(text, top_only=True) if l}
    return names


def plan_one(suffix, modfile, testfile, child, hubname, cfg):
    X.CURRENT[0] = suffix
    set_hosts([modfile])
    text = read(testfile)
    body = FL.rewrite_paths(X.rewrite_heads(text))
    body_nohead = re.sub(r"(?m)^use super::\*;\n", "", body, count=1)
    tmp = SCRATCH + "/ic-" + child + ".rs"
    write(tmp, body_nohead)
    names = module_names(modfile)
    aliases, lines, unresolved = full_imports(suffix, [tmp], local_defs=set())
    own_uses = {l for c, v, p_, l, g in uses(body_nohead, top_only=True) if l}
    uniq = aliases + merge_import_blocks(lines, simplify=True, exclude_locals=names | own_uses, own=lib_name(suffix))
    had_super = "use super::*;" in text
    lines = body_nohead.split("\n")
    k = 0
    while k < len(lines) and (lines[k].startswith("//!") or lines[k].startswith("#![") or not lines[k].strip()):
        k += 1
    lead, rest = "\n".join(lines[:k]).rstrip("\n"), "\n".join(lines[k:])
    head = "use super::*;\n" + ("\n".join(uniq) + "\n" if uniq else "")
    new_text = (lead + "\n\n" if lead else "") + head + "\n" + rest.lstrip("\n")
    return new_text, unresolved


def main():
    added = {}
    only = [a for a in sys.argv[1:] if a not in ('plan', 'apply')]
    for suffix, modfile, testfile, child, hubname, cfg in SPECS:
        if only and child not in only:
            continue
        new_text, unresolved = plan_one(suffix, modfile, testfile, child, hubname, cfg)
        print(os.path.relpath(testfile, E)[-60:], "->", suffix, child, "imports", new_text.count("\nuse "), "ambiguous", unresolved)
        if not APPLY:
            continue
        edit(testfile, new_text)
        mt = read(modfile)
        rel_ = os.path.relpath(testfile, os.path.dirname(modfile))
        mount = f'{cfg}\n#[path = "{rel_}"]\nmod {child};\n'
        if mount not in mt:
            edit(modfile, mt.rstrip("\n") + "\n\n" + mount)
        hub = read(HUB3)
        hub_pat = re.compile(r"(?:#\[cfg[^\n]*\]\n)*#\[path = \"[^\"\n]*\"\]\nmod " + re.escape(hubname) + r";\n")
        found = hub_pat.findall(hub)
        assert len(found) <= 1, (hubname, len(found))
        if found:
            edit(HUB3, hub_pat.sub("", hub))
        added.setdefault(suffix, set()).update(used_idents(new_text))
    if APPLY:
        for suffix, idents in added.items():
            mp = T + "/" + crate_table()[suffix] + "/📦️packages/🦀️rust/Cargo.toml"
            print(suffix, "deps added:", manifest.add_deps(mp, idents, table="dev-dependencies"))


if __name__ == "__main__":
    main()
