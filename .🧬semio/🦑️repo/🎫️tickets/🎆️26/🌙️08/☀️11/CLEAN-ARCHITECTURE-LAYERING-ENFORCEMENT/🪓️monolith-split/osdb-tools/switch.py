"""🔀 The switch: applies the verified stage and all consumer edits to the live repository in one scripted step.

switch.py <dry|apply>   (dry prints what would change and fails on any drift; apply writes)
"""
import filecmp
import hashlib
import json
import os
import re
import shutil
import sys

from lib import *
import register
import facade
from plan import CRATES, BY_LIB

MODE = sys.argv[1] if len(sys.argv) > 1 else 'dry'
WRITE = MODE == 'apply'
log = []


def say(msg):
    log.append(msg)
    print(msg)


def sha_tree(base):
    out = {}
    for root, ds, fs in os.walk(base):
        for f in fs:
            p = os.path.join(root, f)
            out[os.path.relpath(p, base)] = hashlib.sha1(open(p, 'rb').read()).hexdigest()
    return out


def check_no_drift():
    live = sha_tree(DB_REPO)
    pristine = sha_tree(PRISTINE)
    stage = sha_tree(STAGE)
    touched = {k for k in set(pristine) | set(stage) if stage.get(k) != pristine.get(k)}
    drift = sorted(k for k in touched if live.get(k) != pristine.get(k))
    if drift:
        raise SystemExit(f'DRIFT in files the carve touches ({len(drift)} files): {drift[:10]}')
    foreign = sorted(k for k in set(live) | set(pristine) if k not in touched and live.get(k) != pristine.get(k))
    say(f'db tree identical to the snapshot for all {len(touched)} touched files; {len(foreign)} untouched files were changed by peers meanwhile and stay as they are: {foreign[:5]}')


def apply_db_tree():
    stage_files = sha_tree(STAGE)
    pristine = sha_tree(PRISTINE)
    wrote = deleted = 0
    for rel_path, h in sorted(stage_files.items()):
        live = os.path.join(DB_REPO, rel_path)
        if rel_path in pristine and pristine[rel_path] == h:
            continue
        if WRITE:
            os.makedirs(os.path.dirname(live), exist_ok=True)
            shutil.copyfile(os.path.join(STAGE, rel_path), live)
        wrote += 1
    for rel_path in sorted(set(pristine) - set(stage_files)):
        if WRITE:
            os.remove(os.path.join(DB_REPO, rel_path))
        deleted += 1
    say(f'db tree: {wrote} files written, {deleted} files deleted')


def patch_text(path, edits, count_check=True):
    text = read(path)
    for old, new, n in edits:
        c = text.count(old)
        if c != n:
            raise SystemExit(f'{path}: anchor {old[:80]!r} occurs {c} times, expected {n}')
        text = text.replace(old, new)
    return text


def write_live(path, text):
    if WRITE:
        write(path, text)


def apply_registries():
    cargo = f'{REPO}/Cargo.toml'
    new, n = register.update_root_cargo(read(cargo))
    write_live(cargo, new)
    say(f'root Cargo.toml: {n} members added, dead db aliases removed')
    owners_path = f'{REPO}/📋️build-owners.json'
    raw = read(owners_path)
    d, n = register.update_owners(raw)
    out = json.dumps(d, ensure_ascii=False, indent=2) + ('\n' if raw.endswith('\n') else '')
    base = json.dumps(json.loads(raw), ensure_ascii=False, indent=2) + ('\n' if raw.endswith('\n') else '')
    if base != raw:
        raise SystemExit('build-owners.json does not round-trip through json.dumps(indent=2); refusing to rewrite it')
    write_live(owners_path, out)
    say(f'build-owners.json: {n} owner rows added')
    lp, sp = f'{REPO}/.vscode/launch.json', f'{REPO}/.vscode/🧩️launch.seed.jsonc'
    a, b, n = register.update_launch_pair(read(lp), read(sp))
    write_live(lp, a)
    write_live(sp, b)
    say(f'launch.json and seed: {n} routes each')


def apply_hub():
    root, fam, files = facade.scan_hub()
    changed = 0
    for p in files:
        t = read(p)
        t2 = re.sub(r'(?<![A-Za-z0-9_:])db::db_storage::', 'db::storage::', t)
        t2 = re.sub(r'(?<![A-Za-z0-9_:])db::db_engine::', 'db::engine::', t2)
        if t2 != t:
            write_live(p, t2)
            changed += 1
    say(f'hub: {changed} files re-pointed from db::db_storage/db_engine to db::storage/engine')
    hs = f'{HUB_SCRIPT}'
    t = patch_text(hs, [('join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs")', 'join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🦀️.rs")', 1)])
    write_live(hs, t)
    say('hub script: ledger source read from the artifact engine crate')


HUB_SCRIPT = f'{REPO}/🌎️hub/📦️packages/🦀️rust/📜️script.ts'
FAMILIES = {
    'STORAGE': ['🗄️storage/🦀️.rs', '🗄️storage/🧱️io-core/🦀️.rs', '🗄️storage/📇️io-registry/🦀️.rs', '🗄️storage/🚦️io-driver/🦀️.rs', '🗄️storage/🗃️memory/🦀️.rs', '🗄️storage/📁️fs/🦀️.rs'],
    'ENGINE': ['⚙️engine/🦀️.rs', '⚙️engine/🧱️base/🦀️.rs', '⚙️engine/🔭️capability-open/🦀️.rs', '⚙️engine/📖️catalog-read/🦀️.rs', '⚙️engine/🥾️catalog-bootstrap/🦀️.rs', '⚙️engine/🏗️catalog-create/🦀️.rs', '⚙️engine/🎫️handle/🦀️.rs'],
    'ARTIFACT': ['🗿️artifact/🦀️.rs', '🗿️artifact/🧬️contract/🦀️.rs', '🗿️artifact/📜️history/🦀️.rs', '🗿️artifact/⚙️engine/🦀️.rs'],
    'WAL': ['📝️wal/🦀️.rs', '📝️wal/🧾️log/🦀️.rs'],
}


AUDIT_MANUAL = {
    'writer::release::request_retirement(control)': 'crate::request_retirement(control)',
    'pub struct DatabaseCatalogBootstrapKey([u8; 16])': 'pub struct DatabaseCatalogBootstrapKey(pub [u8; 16])',
}


def fix_audit_literals(text):
    """Audit snippets that quote carved db code follow the carve's path rewriting (`db_storage::X` -> `db_io_core::X`)."""
    import json as _json
    from srcfix import collect as _collect
    stage = '\n'.join(_collect(STAGE).values())
    prist = '\n'.join(_collect(PRISTINE).values())
    libs = sorted({c['lib'] for c in _json.load(open(f'{SP}/crates.json'))})
    names = re.findall(r'^(?:export )?function (interactivity(?:Db|Artifact|Database)\w*Failures)\(', text, re.M)
    fixed = unresolved = 0
    for n in names:
        i = text.index('function ' + n + '(')
        ends = [x for x in (text.find('\nfunction ', i + 10), text.find('\nexport function ', i + 10)) if x > 0]
        end = min(ends) if ends else len(text)
        body = text[i:end]
        for m in sorted(re.finditer(r'"((?:[^"\\\n]|\\.)*)"', body), key=lambda m: -m.start()):
            raw = m.group(1)
            lit = raw.replace('\\"', '"').replace('\\n', '\n')
            if len(lit) < 8 or lit not in prist or lit in stage or '\\' in raw.replace('\\"', '').replace('\\n', ''):
                continue
            lit0 = lit
            lit = AUDIT_MANUAL.get(lit, lit.replace('pub(crate) ', 'pub '))
            if lit != lit0 and lit in stage:
                text = text[:i + m.start(1)] + lit.replace('"', '\\"').replace('\n', '\\n') + text[i + m.end(1):]
                fixed += 1
                continue
            cands = [re.sub(r'(?<![A-Za-z0-9_])db(?:_[a-z0-9_]+)?::', lib + '::', lit) for lib in libs]
            cands += [lit.replace(a, b) for a in re.findall(r'(?<![A-Za-z0-9_])db(?:_[a-z0-9_]+)?::', lit) for b in [l + '::' for l in libs]]
            hit = next((c for c in cands if c != lit and c in stage), None)
            if hit is None:
                multi = lit
                for a in sorted(set(re.findall(r'(?<![A-Za-z0-9_])db(?:_[a-z0-9_]+)?::', lit))):
                    for lib in libs:
                        t = multi.replace(a, lib + '::')
                        if t in stage:
                            multi = t
                            break
                hit = multi if multi != lit and multi in stage else None
            if hit is None:
                unresolved += 1
                say(f'audit literal not found in the carved tree: {lit[:100]!r}')
                continue
            new_raw = hit.replace('"', '\\"').replace('\n', '\\n')
            a, b = i + m.start(1), i + m.end(1)
            text = text[:a] + new_raw + text[b:]
            fixed += 1
    cli = text.index('function interactivityDbCliFailures(')
    cli_end = text.index('\n}\n', cli)
    text = text[:cli] + text[cli:cli_end].replace('db::actor::block_on', 'db_actor::block_on') + text[cli_end:]
    say(f'root script.ts: {fixed} audit snippets re-quoted to the carved paths, {unresolved} unresolved')
    return text


def apply_root_script():
    path = f'{REPO}/📜️script.ts'
    text = read(path)
    anchor = 'const INTERACTIVITY_AUDIT_DB_WAL_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs";\n'
    assert text.count(anchor) == 1
    block = ''
    for k, paths in FAMILIES.items():
        items = ', '.join(f'"{DB_REL}/{p}"' for p in paths)
        block += f'const INTERACTIVITY_AUDIT_DB_{k}_FAMILY = [{items}] as const;\n'
    text = text.replace(anchor, anchor + block)
    for k in FAMILIES:
        for recv in ('this.root', 'repoRoot'):
            old = f'policyReadRustPolicySource({recv}, INTERACTIVITY_AUDIT_DB_{k}_FILE)'
            new = f'policyReadRustPolicyFamily({recv}, INTERACTIVITY_AUDIT_DB_{k}_FAMILY)'
            text = text.replace(old, new)
    fn_anchor = 'function policyReadRustPolicySource(repoRoot: string, sourcePath: string): string {'
    assert text.count(fn_anchor) == 1
    helper = ('/** 🧩️ Source of a crate family: the production source of every crate root of one carved db module first, then the test evidence of all of them. */\n'
              'function policyReadRustPolicyFamily(repoRoot: string, roots: readonly string[]): string {\n'
              '  const evidence = roots.map((root) => policyReadRustSourceEvidence(repoRoot, root));\n'
              '  return evidence.map((entry) => entry.production.source).join("\\n") + evidence.flatMap((entry) => entry.tests).map((entry) => `${POLICY_RUST_TEST_EVIDENCE_BOUNDARY}${entry.path}\\n${entry.source}`).join("");\n}\n\n')
    text = text.replace(fn_anchor, helper + fn_anchor)
    text = fix_audit_literals(text)
    write_live(path, text)
    say('root script.ts: four db audit readers read the whole crate family')


def apply_os_script():
    path = f'{REPO}/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts'
    sq = BY_LIB['db_storage']
    pg = BY_LIB['db_storage_postgres']
    old1 = 'join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml"), "--features", "sqlite,postgres,neo4j", "--lib", "--", "--exact", WAL_WRITER_FENCE_LANES[lane]!.law,'
    new1 = f'join(this.repoRoot, "{DB_REL}/{sq.pkg_dir}/Cargo.toml"), "--lib", "--", "--exact", WAL_WRITER_FENCE_LANES[lane]!.law,'
    old2 = 'join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml"), "--features", "sqlite,postgres", "--lib", "--", "db_storage_postgres::round_trips::",'
    new2 = f'join(this.repoRoot, "{DB_REL}/{pg.pkg_dir}/Cargo.toml"), "--lib", "--", "round_trips::",'
    t = patch_text(path, [(old1, new1, 1), (old2, new2, 1)])
    write_live(path, t)
    say('os script.ts: live-law cargo commands point at the carved crates')


def apply_boundary():
    base = f'{TOOLS}/boundary-files'
    n = 0
    for root, ds, fs in os.walk(base):
        for f in fs:
            src = os.path.join(root, f)
            dest = os.path.join(DB_REPO, os.path.relpath(src, base))
            if WRITE:
                os.makedirs(os.path.dirname(dest), exist_ok=True)
                shutil.copyfile(src, dest)
            n += 1
    fixture = os.path.join(DB_REPO, '🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json')
    if WRITE:
        os.makedirs(os.path.dirname(fixture), exist_ok=True)
        shutil.copyfile(f'{SP}/boundaries.json', fixture)
    say(f'crate-boundary test: {n} authored files plus the generated corpus')


def main():
    check_no_drift()
    apply_db_tree()
    apply_boundary()
    apply_registries()
    apply_hub()
    apply_root_script()
    apply_os_script()
    say('DONE ' + MODE)


if __name__ == '__main__':
    main()
