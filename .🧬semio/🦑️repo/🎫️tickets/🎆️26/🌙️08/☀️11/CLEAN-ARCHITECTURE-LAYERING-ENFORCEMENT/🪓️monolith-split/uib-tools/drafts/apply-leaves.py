import os, re, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
FW = f"{ROOT}/🧰️framework/🔨️modules"
UIM = f"{FW}/🖱️ui/🔨️modules"
PLAT = f"{FW}/🖥️platform/🟦️.ts"
DRAFTS = os.path.dirname(os.path.abspath(__file__))
apply = "--apply" in sys.argv

def read(p):
    with open(p, encoding="utf8") as f: return f.read()
def write(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf8") as f: f.write(s)
def cut(text, start, end_inclusive_marker):
    assert text.count(start) == 1, f"start anchor not unique: {start[:60]!r} x{text.count(start)}"
    i = text.index(start)
    j = text.index(end_inclusive_marker, i)
    assert text.count(end_inclusive_marker, i) >= 1
    return i, j + len(end_inclusive_marker)

plat = read(PLAT)

# 1. element ids ---------------------------------------------------------
ids_start = "/** 🆔️ Normalizes arbitrary input into a single camelCase element-id segment"
ids_end = "return `framework.panelTab.${tabId}.firstDraggable`;\n}\n"
i, j = cut(plat, ids_start, ids_end)
ids_block = plat[i:j]
plat = plat[:i].rstrip("\n") + "\n" + plat[j:]

# 2. storage interface ---------------------------------------------------
sp_start = "export interface StoragePort {\n  get(key: string): string | null;\n  set(key: string, value: string): void;\n  remove(key: string): void;\n}\n\n"
assert plat.count(sp_start) == 1
plat = plat.replace(sp_start, "")

# 3. storage factories ---------------------------------------------------
f_start = "export function createBrowserStoragePort(): StoragePort {"
f_end = "    remove: (key) => base.remove(`${prefix}${key}`),\n  };\n}\n\n"
i, j = cut(plat, f_start, f_end)
factories_block = plat[i:j]
plat = plat[:i] + plat[j:]
plat = re.sub(r"\n{4,}", "\n\n\n", plat) if False else plat

# 4. dock skeleton -------------------------------------------------------
pa_start = "/** 🧭️ The eight anchor ids, mirroring `Anchor` in `framework/ui/js/react/index.tsx`"
pa_end = '"bottom-left" | "left-middle";\n\n'
i, j = cut(plat, pa_start, pa_end)
plat = plat[:i] + plat[j:]
dk_start = "/** 🐳️ One tab (leaf or branch) in a persisted dock panel-arrangement tree;"
dk_end = "  anchors: Record<PersistedAnchor, readonly DockTabSkeleton[]>;\n}\n\n"
i, j = cut(plat, dk_start, dk_end)
plat = plat[:i] + plat[j:]

# 5. imports in platform ---------------------------------------------------
imp_anchor = 'import type { WindowLayout, WindowLayoutWindowNode } from "../🛂️manifest/🗺️window-layout/🟦️.ts";\n'
assert plat.count(imp_anchor) == 1
plat = plat.replace(imp_anchor, imp_anchor + 'import type { DockSkeleton, PersistedAnchor } from "../🖱️ui/🔨️modules/🐳️dock-skeleton/🟦️.ts";\nimport type { StoragePort } from "../🖱️ui/🔨️modules/🗄️storage-port/🟦️.ts";\n')

# leaves --------------------------------------------------------------------
storage_leaf = '''/** 🗄️ Device-local key-value storage port with browser, in-memory and namespaced implementations. */

/** 🗄️ Synchronous string storage a shell persists its device-local configuration through. */
export interface StoragePort {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
}

/** 🌐️ {@link StoragePort} over `localStorage`; a blocked or absent storage degrades to an empty one. */
export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
      } catch {}
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
      } catch {}
    },
  };
}

/** 🧠️ In-memory {@link StoragePort} — used by ephemeral branded shells so nothing survives a window refresh. */
export function createMemoryStoragePort(): StoragePort {
  const map = new Map<string, string>();
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    },
  };
}

/** 🐚️ Namespaces a {@link StoragePort} under `semio.shell.<namespace>.` so several shell instances sharing one
 * browser storage origin (e.g. several demonstrator panes) don't read/write each other's `semio.os.dock`/`ui.chrome.*`
 * keys. Not needed for a single page-owning shell — that shell's default (unprefixed) storage is the intended shared surface. */
export function createScopedStoragePort(base: StoragePort, namespace: string): StoragePort {
  const prefix = `semio.shell.${namespace}.`;
  return {
    get: (key) => base.get(`${prefix}${key}`),
    set: (key, value) => base.set(`${prefix}${key}`, value),
    remove: (key) => base.remove(`${prefix}${key}`),
  };
}
'''
# verify the factories moved verbatim except the two comment-only catch bodies
norm = lambda s: re.sub(r"\s+", " ", re.sub(r"/\*.*?\*/|/\*\*.*?\*/", "", s, flags=re.S)).replace("catch { } ", "catch {} ").replace("{ } }", "{}}")
dock_leaf = '''/** 🐳️ Persisted dock arrangement vocabulary shared by the dock layout store and the dock renderers. */

/** 🧭️ The eight anchor ids, mirroring `Anchor` in `🖱️ui/🔨️modules/⚓️panel-anchor` (kept dependency-free of that React module) — shared by every persisted anchor-keyed shape so they cannot drift apart from one another. */
export type PersistedAnchor = "top-left" | "top-middle" | "top-right" | "right-middle" | "bottom-right" | "bottom-middle" | "bottom-left" | "left-middle";

/** 🐳️ One tab (leaf or branch) in a persisted dock panel-arrangement tree; leaves carry `trees`, branches carry `children`. */
export interface DockTabSkeleton {
  id: string;
  children?: readonly DockTabSkeleton[];
  trees?: readonly string[];
}

/** 🐳️ The full persisted dock arrangement, one tab tree per anchor. */
export interface DockSkeleton {
  version: 3;
  anchors: Record<PersistedAnchor, readonly DockTabSkeleton[]>;
}
'''
EI = f"{UIM}/🆔️element-identity/🟦️.ts"
ei = read(EI)
ei_end = "// #endregion 🆔️ElementIdentity\n"
assert ei.count(ei_end) == 1
ei_new = ei.replace(ei_end, ei_end + "\n// #region 🆔️ElementIds\n" + ids_block.rstrip("\n") + "\n// #endregion 🆔️ElementIds\n")
ephemeral = read(f"{DRAFTS}/ephemeral-state.ts")

print("platform lines:", plat.count("\n"), "ids block", ids_block.count("\n"), "factories block", factories_block.count("\n"))
for name, path in (("storage", f"{UIM}/🗄️storage-port/🟦️.ts"), ("dock", f"{UIM}/🐳️dock-skeleton/🟦️.ts"), ("ephemeral", f"{UIM}/🫧️ephemeral-state/🟦️.ts")):
    assert not os.path.exists(path), f"{path} exists"
if apply:
    write(PLAT, plat)
    write(f"{UIM}/🗄️storage-port/🟦️.ts", storage_leaf)
    write(f"{UIM}/🐳️dock-skeleton/🟦️.ts", dock_leaf)
    write(f"{UIM}/🫧️ephemeral-state/🟦️.ts", ephemeral)
    write(EI, ei_new)
    print("applied")
else:
    print("dry-run ok")
