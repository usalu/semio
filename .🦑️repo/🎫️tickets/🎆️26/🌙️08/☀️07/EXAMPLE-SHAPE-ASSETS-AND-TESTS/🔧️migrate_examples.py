#!/usr/bin/env python3
"""Ticket-local example-shape migrator."""
from __future__ import annotations
from pathlib import Path
import json, shutil, re, sys

ROOT = Path(__file__).resolve().parents[5]
# parents: script -> ticket -> day -> month -> year -> tickets -> .repo — adjust
# Actually: .../EXAMPLE.../script -> ticket dir is parent
TICKET = Path(__file__).resolve().parent
ROOT = TICKET
while ROOT.name != 'semio' and ROOT != ROOT.parent:
    ROOT = ROOT.parent
paths = json.loads(next((ROOT/'.🦑️repo').rglob('🧭️paths.json')).read_text())
TAX = json.loads(Path(paths['taxonomy']).read_text())
ASSETS = TAX['exampleAssetsDirName']
TESTS = TAX['exampleTestsDirName']
TS_LEAF = TAX['exampleLeafFilenames']['🟦️typescript']
RS_LEAF = TAX['exampleLeafFilenames']['🦀️rust']
FORBIDDEN = set(TAX.get('forbiddenExampleSlugs') or [])
PLURAL = {
  '🗣️dsls': ('🗣️', 'dsl'),
  '🔧️ops': ('🔧️', 'op'),
  '📡️sprs': ('📡️', 'spr'),
  '🎒️packs': ('🎒️', 'pack'),
}

def emoji_slug(name: str, default_emoji='🎬️') -> str:
  if any(ord(c) > 0x2000 for c in name[:4]):
    return re.sub(r'(\ufe0f)+', '\ufe0f', name)
  base = default_emoji.rstrip('\ufe0f')
  return base + '\ufe0f' + name

def migrate_slug(slug_dir: Path, asset_basename: str):
  assets = slug_dir / ASSETS
  tests = slug_dir / TESTS
  assets.mkdir(exist_ok=True)
  tests.mkdir(exist_ok=True)
  for plural, (prefix, kind) in PLURAL.items():
    pdir = slug_dir / plural
    if not pdir.is_dir():
      continue
    for leaf in pdir.rglob('*.semio'):
      target = assets / f'{prefix}{asset_basename}.{kind}.semio'
      n = 2
      while target.exists():
        target = assets / f'{prefix}{asset_basename}-v{n}.{kind}.semio'
        n += 1
      shutil.copy2(leaf, target)
    shutil.rmtree(pdir)
  for rs in list(slug_dir.rglob('🦀️component.rs')):
    if rs.parent == slug_dir:
      continue
    if ASSETS in rs.parts or TESTS in rs.parts:
      continue
    rs.unlink(missing_ok=True)
  for leaf in list(slug_dir.rglob('*.cmd.semio')):
    if ASSETS in leaf.parts:
      continue
    target = assets / f'🎮️{asset_basename}.cmd.semio'
    if not target.exists():
      shutil.copy2(leaf, target)

def ensure_leaves(slug_dir: Path, example_id: str, label_en: str, label_de: str, icon: str, primary_glob='*'):
  assets = slug_dir / ASSETS
  tests = slug_dir / TESTS
  assets.mkdir(exist_ok=True)
  tests.mkdir(exist_ok=True)
  primary = None
  for pattern in ('🗣️*.dsl.semio', '🎮️*.cmd.semio', '*.semio'):
    hits = list(assets.glob(pattern))
    if hits:
      primary = hits[0]
      break
  if primary is None:
    primary = assets / '🗣️example.dsl.semio'
    primary.write_text(f'semio {example_id}.dsl v1\nid={example_id}\nbody=demo\n')
  rel = f'{ASSETS}/{primary.name}'
  rs = slug_dir / RS_LEAF
  if not rs.exists():
    rs.write_text(f'''//! 📚️ Example `{example_id}`.

use semio_framework_os_kernel::plugin::ExampleSource;
use semio_framework::LocalizedLabel;

pub const ID: &str = "{example_id}";
pub fn label() -> LocalizedLabel {{ LocalizedLabel::native("{label_en}", "{label_de}") }}
pub const ICON: &str = "{icon}";
pub const PRIMARY_TEXT: &str = include_str!("{rel}");
pub fn source() -> ExampleSource {{ ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }}
''')
  ts = slug_dir / TS_LEAF
  if not ts.exists():
    ts.write_text(f'''/** 📚️ Example `{example_id}`. */
export const id = "{example_id}";
export const label = {{ en: "{label_en}", de: "{label_de}" }} as const;
export const icon = "{icon}";
''')
  trs = tests / '🦀️test.rs'
  if not trs.exists():
    trs.write_text(f'''#[test]
fn primary_asset_is_nonempty() {{
    let text = include_str!("../{rel}");
    assert!(text.len() > 8);
}}
''')
  tts = tests / '🟦️test.ts'
  if not tts.exists():
    tts.write_text(f'''import {{ readFileSync }} from "node:fs";
import {{ dirname, join }} from "node:path";
import {{ fileURLToPath }} from "node:url";
import {{ describe, expect, it }} from "vitest";
const here = dirname(fileURLToPath(import.meta.url));
describe("{example_id}", () => {{
  it("ships primary asset", () => {{
    expect(readFileSync(join(here, "../{rel}"), "utf8").length).toBeGreaterThan(8);
  }});
}});
''')

def ascii_id(name: str) -> str:
  s = name.encode('ascii', 'ignore').decode()
  s = re.sub(r'[^a-zA-Z0-9]+', '-', s).strip('-').lower()
  return s or 'example'

def migrate_plugin(plugin_dir: Path, default_emoji='🎬️'):
  log = []
  arts = plugin_dir / '🗿️artifacts'
  if arts.is_dir():
    for art in arts.iterdir():
      if not art.is_dir():
        continue
      ex = art / '📚️examples'
      ex.mkdir(exist_ok=True)
      for slug in list(ex.iterdir()):
        if not slug.is_dir():
          continue
        if slug.name in FORBIDDEN:
          shutil.rmtree(slug)
          log.append({'delete': str(slug)})
          continue
        new = emoji_slug(slug.name, default_emoji)
        dest = ex / new
        if new != slug.name:
          if dest.exists() and dest != slug:
            shutil.rmtree(dest)
          slug.rename(dest)
          slug = dest
        base = ascii_id(new)
        migrate_slug(slug, base)
        ensure_leaves(slug, base, base.replace('-', ' ').title(), base.replace('-', ' ').title(), 'file')
      if not any(p.is_dir() for p in ex.iterdir()):
        slug = ex / (default_emoji + '\ufe0fdemo')
        slug.mkdir(parents=True)
        ensure_leaves(slug, 'demo', 'Demo', 'Demo', 'file')
        log.append({'seed': str(slug)})
  apps = plugin_dir / '🎛️apps'
  if apps.is_dir():
    for app in apps.iterdir():
      if not app.is_dir():
        continue
      eng = app / '⚙️engine' / '📚️examples'
      dest_root = app / '📚️examples'
      dest_root.mkdir(exist_ok=True)
      if eng.is_dir():
        cmds = list(eng.rglob('*.cmd.semio'))
        new_slug = dest_root / (default_emoji + '\ufe0fdemo-session')
        if new_slug.exists():
          shutil.rmtree(new_slug)
        new_slug.mkdir(parents=True)
        (new_slug / ASSETS).mkdir(exist_ok=True)
        if cmds:
          shutil.copy2(cmds[0], new_slug / ASSETS / '🎮️demo.cmd.semio')
        else:
          (new_slug / ASSETS / '🎮️demo.cmd.semio').write_text('semio demo.cmd v1\naction=demo\n')
        ensure_leaves(new_slug, 'demo-session', 'Demo Session', 'Demo-Sitzung', 'play')
        shutil.rmtree(eng)
        log.append({'app': str(new_slug)})
      if not any(p.is_dir() for p in dest_root.iterdir()):
        new_slug = dest_root / (default_emoji + '\ufe0fdemo-session')
        new_slug.mkdir(parents=True)
        ensure_leaves(new_slug, 'demo-session', 'Demo Session', 'Demo-Sitzung', 'play')
  root_ex = plugin_dir / '📚️examples'
  if root_ex.is_dir() and arts.is_dir():
    first = next((a for a in arts.iterdir() if a.is_dir()), None)
    if first:
      target = first / '📚️examples'
      target.mkdir(exist_ok=True)
      for child in list(root_ex.iterdir()):
        dest = target / child.name
        if dest.exists():
          shutil.rmtree(dest) if dest.is_dir() else dest.unlink()
        shutil.move(str(child), str(dest))
      shutil.rmtree(root_ex)
      # remigrate moved
      for slug in list(target.iterdir()):
        if slug.is_dir() and slug.name not in FORBIDDEN:
          migrate_slug(slug, ascii_id(slug.name))
          ensure_leaves(slug, ascii_id(slug.name), 'Demo', 'Demo', 'file')
        elif slug.is_dir() and slug.name in FORBIDDEN:
          shutil.rmtree(slug)
      log.append({'moved_root': str(target)})
  return log

if __name__ == '__main__':
  plugins = sys.argv[1:]
  base = ROOT / '✏️s' / '🔌️plugins'
  for name in plugins:
    matches = [p for p in base.iterdir() if name in p.name]
    if not matches:
      print('missing', name)
      continue
    log = migrate_plugin(matches[0])
    print(matches[0].name, 'ops', len(log))
