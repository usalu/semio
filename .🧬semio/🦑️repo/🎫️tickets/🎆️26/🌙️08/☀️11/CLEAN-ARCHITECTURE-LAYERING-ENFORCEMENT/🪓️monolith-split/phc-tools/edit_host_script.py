#!/usr/bin/env python3
"""📜️ The host script after the carve: family-wide check and test, per-package lifecycle discovery, re-pointed law groups and source readers."""
import os, sys, re, difflib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, lawmap, tsfix_host, plan_host
from carve import REPO

REL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts'
FAMILY = [v['name'] for v in plan_host.CRATES.values()]


def once(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:90])
    return text.replace(old, new)


def edit(text, table):
    text, skipped, unmapped = tsfix_host.rewrite_text(text, table)
    assert not skipped and not unmapped, (skipped, unmapped)
    names = ',\n  '.join(f'"{n}"' for n in FAMILY)
    text = once(text, '//#region 🎯️Tasks\nclass CheckScript', f'//#region 🎯️Tasks\n/** 🧱️ The facade package and every producer crate it is assembled from. */\nconst HOST_FAMILY = [\n  {names},\n] as const;\nconst familyPackages = HOST_FAMILY.flatMap((name) => ["-p", name]);\n\nclass CheckScript')
    text = once(text, 'await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);', 'await runCargo(["check", "--manifest-path", "Cargo.toml", ...familyPackages, ...segments], this.root);')
    text = once(text, 'await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);', 'await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...familyPackages, ...segments], this.root);')
    # original driver policy group (template law names) -> wake crate
    text = once(text, 'groups:[{package:"semio-framework-plugin-host",target:{kind:"lib"},laws:names.map(name=>`component::original_wake_receipts::tests::${name}`)}]', 'groups:[{package:"semio-framework-plugin-host-wake",target:{kind:"lib"},laws:names.map(name=>`tests::${name}`)}]')
    # owned instance check runs the guest laws crate
    text = once(text, '"-p","semio-framework-plugin-host","--lib","owned_instance_open_tests"', '"-p","semio-framework-plugin-host-guest-laws","--lib","owned_instance_open_tests"')
    # source readers
    text = once(text, 'const synchronous = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");\n    const asynchronous', 'const synchronous = readFileSync(join(hostRoot, "🔗️wit-bindings", "🔀️conversion", "🦀️.rs"), "utf8");\n    const asynchronous')
    text = once(text, '  const host = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");\n  const retained', '  const host = readFileSync(join(hostRoot, "🎭️guest-runtime", "🦀️.rs"), "utf8");\n  const retained')
    text = once(text, 'const synchronous = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");\n    const scale', 'const synchronous = readFileSync(join(hostRoot, "🎭️guest-runtime", "🦀️.rs"), "utf8");\n    const scale')
    a = text.index('    const focused = [')
    endmark = '    await runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features"], this.root);\n'
    b = text.index(endmark, a) + len(endmark)
    block = text[a:b]
    old_laws = re.findall(r'"((?:[^"\\]|\\.)*)"', block[:block.index('];')])
    mapped = []
    for law in old_laws:
        r = lawmap.map_law(table, law)
        assert r is not None, law
        mapped.append((plan_host.CRATES[r[0]]['name'], r[1]))
    rows = ',\n      '.join(f'["{p}", "{l}"]' for p, l in mapped)
    new_block = f'''    const focused: readonly (readonly [string, string])[] = [
      {rows},
    ];
    const discoveredByPackage = new Map<string, string[]>();
    const discover = (pkg: string): string[] => {{
      const known = discoveredByPackage.get(pkg);
      if (known) return known;
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", "-p", pkg, "--", "--list"], {{ cwd: this.root, ...orchestratorBudgetOpts() }});
      if (listed.status !== 0) throw new Error(`plugin-host lifecycle inventory of ${{pkg}} failed with status ${{listed.status}}`);
      const names = listed.stdout
        .split("\\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length));
      discoveredByPackage.set(pkg, names);
      return names;
    }};
    const laws = focused.map(([pkg, suffix]) => {{
      const matches = discover(pkg).filter((name) => name.endsWith(suffix));
      if (matches.length !== 1) throw new Error(`plugin-host lifecycle gate expected exactly one ${{suffix}} law in ${{pkg}}, selected ${{matches.length}}`);
      return [pkg, matches[0]!] as const;
    }});
    console.log(`plugin-host-lifecycle-laws: ${{laws.map(([pkg, law]) => `${{pkg}}:${{law}}`).join(" ")}}`);
    for (const [pkg, law] of laws) await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "-p", pkg, law, "--", "--exact"], this.root);
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...familyPackages], this.root);
    await runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", ...familyPackages], this.root);
'''
    text = text[:a] + new_block + text[b:]
    return text


if __name__ == '__main__':
    ws, model, roots = state.load()
    table = lawmap.build_map(model, state.base_ws(ws))
    p = os.path.join(REPO, REL)
    text = open(p, encoding='utf-8').read()
    new = edit(text, table)
    for l in difflib.unified_diff(text.split('\n'), new.split('\n'), lineterm='', n=0):
        if l.startswith(('+', '-')) and not l.startswith(('+++', '---')):
            print(l[:200])
    if '--apply' in sys.argv:
        if open(p, encoding='utf-8').read() != text:
            raise RuntimeError('concurrent edit')
        open(p, 'w', encoding='utf-8').write(new)
