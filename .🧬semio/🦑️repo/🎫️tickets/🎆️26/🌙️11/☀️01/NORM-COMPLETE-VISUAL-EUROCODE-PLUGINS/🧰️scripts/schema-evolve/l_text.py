"""📝 Mutation text codec of artifacts that keep it per family (`🚪️io/📝️text/🧬️mutations/<family>/`: a kit crate plus one crate per family, a dispatcher with one print arm and one parse arm per kind): the print/parse pairs of the new kinds are cloned from the donor kinds into a new family crate `mutations-text-<plural>` (manifest, workspace member and dependency, `script.ts` package, dependency of the dispatcher crate), and the two dispatcher arms per kind are routed to it. Artifacts with a monolithic codec need nothing here: the registry clone already copied the arms and functions."""
import os, re
from core import Unsupported, kebab, pascal, snake, item_at, walk_files

TEXT = "🚪️io/📝️text/🧬️mutations"
KIT = re.compile(r'^name = "([^"]+-mutations-text-kit)"', re.M)


def families(ctx):
    """🏘️ (directory, crate name) of every family crate below the text dispatcher, and the kit crate name."""
    root = ctx.layout.p(TEXT)
    kit, fams = None, []
    for d in sorted(os.listdir(root)):
        manifest = f"{root}/{d}/📦️packages/🦀️rust/Cargo.toml"
        if os.path.isfile(f"{root}/{d}/🦀️.rs") and os.path.isfile(manifest):
            name = re.search(r'^name = "([^"]+)"', ctx.plan.read(manifest), re.M).group(1)
            if KIT.search(ctx.plan.read(manifest)):
                kit = name
            else:
                fams.append((f"{root}/{d}", name))
    return kit, fams


def donor_fn(ctx, fams, ref_kind):
    """🔎 The family file holding the print function of the donor kind."""
    fn = "print_" + snake(ref_kind)
    for d, name in fams:
        text = ctx.plan.read(f"{d}/🦀️.rs")
        if re.search(r"^pub fn %s\(" % fn, text, re.M):
            return d, name, text
    raise Unsupported(f"the text codec of {ref_kind} is in no family crate below {TEXT}")


def clone_fns(text, ref_kind, translate):
    out = ""
    for verb in ("print", "parse"):
        a, b = item_at(text, r"^pub fn %s_%s\(" % (verb, snake(ref_kind)))
        out += translate(text[a:b]).rstrip("\n") + "\n\n"
    return out


def into_donor_family(ctx, fams, pairs):
    """🧷 A leaf (new set-field kind of an existing entity) joins the family crate of its donor kind; the dispatcher arms already name that family."""
    for ref_kind, new_kind, translate in pairs:
        d, dname, dtext = donor_fn(ctx, fams, ref_kind)
        path = d + "/🦀️.rs"
        text = ctx.plan.read(path)
        if f"pub fn print_{snake(new_kind)}(" in text:
            continue
        m = re.search(r"^use (semio_\w+)::%s::[^\n]*\n" % snake(ref_kind), text, re.M)
        if not m:
            raise Unsupported(f"the family crate of {ref_kind} does not import the kind from its mutation crate")
        use = f"use {m.group(1)}::{snake(new_kind)}::{pascal(new_kind)};\n"
        last = list(re.finditer(r"^use semio_\w+_mut_[^\n]*\n", text, re.M))[-1]
        text = text[: last.end()] + use + text[last.end():]
        ctx.plan.write(path, text.rstrip("\n") + "\n\n" + clone_fns(text, ref_kind, translate).rstrip("\n") + "\n")
    ctx.plan.note(f"text codec: print + parse of {len(pairs)} kind(s) added to the family crate of the donor kind")


def run(ctx, crate):
    pairs = getattr(ctx, "text_pairs", [])
    root = ctx.layout.p(TEXT)
    if not pairs or not os.path.isdir(root):
        return
    kit, fams = families(ctx)
    if not kit:
        ctx.plan.note("text codec: monolithic, the cloned arms and functions of the registry clone are the codec of the new kinds")
        return
    S = ctx.spec.collection if ctx.spec.collection else None
    if S is None:
        return into_donor_family(ctx, fams, pairs)
    slug = kebab(S.plural)
    ndir = f"{root}/{S.emoji}{slug}"
    dispatcher = root + "/🦀️.rs"
    uses, fns, crates = set(), "", set()
    existing = ctx.plan.read(ndir + "/🦀️.rs") if ctx.plan.exists(ndir + "/🦀️.rs") else ""
    for ref_kind, new_kind, translate in pairs:
        d, dname, dtext = donor_fn(ctx, fams, ref_kind)
        if f"pub fn print_{snake(new_kind)}(" in existing:
            continue
        m = re.search(r"^use (semio_\w+)::%s::" % snake(ref_kind), dtext, re.M)
        if not m:
            raise Unsupported(f"the family crate of {ref_kind} does not import the kind from its mutation crate")
        home = crate.replace("-", "_") if crate else m.group(1)
        crates.add(home)
        uses.add(f"use {home}::{snake(new_kind)}::{pascal(new_kind)};\n")
        fns += clone_fns(dtext, ref_kind, translate)
    if not fns:
        return
    d0, dname0, dtext0 = donor_fn(ctx, fams, pairs[0][0])
    kit_use = re.search(r"^use semio_\w+_mutations_text_kit::[^\n]*\n", dtext0, re.M).group(0)
    head = f"//! {S.emoji} {S.entity} mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.\n\n#![allow(unused_imports)]\n\n{kit_use}"
    ctx.plan.write(ndir + "/🦀️.rs", head + "".join(sorted(uses)) + "\n" + fns.rstrip("\n") + "\n")
    new_name = re.sub(r"-text-[\w-]+$", "-text-" + slug, dname0)
    alias = "text_" + snake(slug)
    mut_crates = sorted(c.replace("_", "-") for c in crates)
    manifest = ctx.plan.read(f"{d0}/📦️packages/🦀️rust/Cargo.toml")
    deps = manifest.index("[dependencies]\n") + len("[dependencies]\n")
    kit_dep = f"{kit} = {{ workspace = true }}\n"
    new_manifest = manifest[:deps].replace(dname0, new_name) + kit_dep + "".join(f"{c} = {{ workspace = true }}\n" for c in mut_crates)
    new_manifest = re.sub(r'^description = ".*"$', f'description = "{S.entity} mutation text codec"', new_manifest, flags=re.M)
    ctx.plan.write(ndir + "/📦️packages/🦀️rust/Cargo.toml", new_manifest)
    text = ctx.plan.read(dispatcher)
    donor_alias = re.search(r"^use %s as (text_\w+);\n" % re.escape(dname0.replace("-", "_")), text, re.M)
    if not donor_alias:
        raise Unsupported("the dispatcher has no `use <family crate> as text_..` line of the donor family")
    use_line = f"use {new_name.replace('-', '_')} as {alias};\n"
    if use_line not in text:
        text = text[: donor_alias.end()] + use_line + text[donor_alias.end():]
    for _, new_kind, _ in pairs:
        for verb in ("print", "parse"):
            fn = f"{verb}_{snake(new_kind)}"
            if not re.search(r"\w+::%s\(" % fn, text):
                raise Unsupported(f"the dispatcher has no {verb} arm for {new_kind}")
            text = re.sub(r"\btext_\w+::%s\(" % fn, f"{alias}::{fn}(", text)
    ctx.plan.write(dispatcher, text)
    register(ctx, dname0, new_name, ndir)
    ctx.plan.note(f"text codec: {len(pairs)} kinds in the new family crate {new_name} (print + parse each), {2 * len(pairs)} dispatcher arms routed to {alias}")


def register(ctx, donor, new, ndir):
    """🧾 Workspace member and dependency, `script.ts` package and the dependency of the dispatcher crate."""
    art = ctx.layout.dir
    for path in walk_files(art, ("Cargo.toml", "script.ts")):
        if "/target/" in path or path.startswith(ndir + "/"):
            continue
        text = ctx.plan.read(path)
        new_text = text
        if path.endswith("script.ts") and f'"{donor}"' in text and f'"{new}"' not in text:
            new_text = text.replace(f'"{donor}"', f'"{donor}","{new}"', 1)
        elif path == art + "/Cargo.toml":
            donor_dir = None
            blk = re.search(r'\[workspace\.dependencies\.%s\]\npath = "([^"]+)"\n\n' % re.escape(donor), text)
            if blk and f"[workspace.dependencies.{new}]" not in text:
                donor_dir = blk.group(1)
                ndir_rel = os.path.relpath(ndir, art) + "/📦️packages/🦀️rust"
                new_text = text[: blk.end()] + f'[workspace.dependencies.{new}]\npath = "{ndir_rel}"\n\n' + text[blk.end():]
                member = f'    "{donor_dir}",\n'
                if member in new_text:
                    new_text = new_text.replace(member, member + f'    "{ndir_rel}",\n', 1)
        elif f"{donor} = {{ workspace = true }}\n" in text and f"{new} =" not in text and "[dependencies]" in text:
            new_text = text.replace(f"{donor} = {{ workspace = true }}\n", f"{donor} = {{ workspace = true }}\n{new} = {{ workspace = true }}\n", 1)
        if new_text != text:
            ctx.plan.write(path, new_text)
