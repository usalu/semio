# 🧺️ Bundle conversion: `impl ShellState` use-case blocks of the WP5 staging crates become borrowed-bundle impls, calls into lower WP5 crates go through explicit `as_<crate>` bridges, self-less associated functions become free functions.
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, rsedit, mover, deps
from rsedit import Doc
from blank import blank

COMPONENTS = [("chrome", "ChromeState"), ("surfaces", "SurfaceState"), ("tutorial", "TutorialState"), ("settle", "SettleState"), ("plugins", "PluginState"), ("history", "HistoryState"),
              ("prefs", "PrefsState"), ("directory", "DirectoryState"), ("sync", "SyncState"), ("docs", "DocumentState"), ("opening", "OpeningState"), ("windows", "WindowState"),
              ("dock", "ShellDockState"), ("input", "ShellInputState"), ("commands", "CommandState")]
BRIDGE = {"shell-session": "as_session", "shell-window-actions": "as_window_actions", "shell-actions": "as_actions", "shell-pointer": "as_pointer",
          "shell-view-chrome": "as_chrome_view", "shell-view-overlays": "as_overlays_view", "shell-view-panels": "as_panels_view"}
BUNDLE_EMOJI = {"shell-session": "🎒", "shell-window-actions": "🧳", "shell-actions": "👜", "shell-pointer": "🛍", "shell-view-chrome": "💼", "shell-view-overlays": "👝", "shell-view-panels": "🎽"}
BUNDLE_SLUG = "🧺️bundle"

def match_close(b, i):
    d = 0
    while i < len(b):
        if b[i] in "([{": d += 1
        elif b[i] in ")]}":
            d -= 1
            if d == 0: return i
        i += 1
    return -1

def crate_files(root, crate):
    base = cfg.stg(root, cfg.CRATE2DIR[crate]); out = []
    if os.path.isdir(base):
        for slug in sorted(os.listdir(base)):
            p = f"{base}/{slug}/🦀️.rs"
            if os.path.exists(p) and slug != BUNDLE_SLUG: out.append((slug, p))
    return out

def method_homes(root):
    ns = deps.load(root); homes = {}
    for n in ns:
        if n.kind == "method": homes.setdefault(n.name, set()).add(n.crate)
    return homes

def bundle_text(crate, bridges):
    x = cfg.CTX[crate]; em = BUNDLE_EMOJI[crate]
    L = [f"//! {em}️ `{crate}` bundle: `{x}`, the borrowed split of the `ShellState` composer that every use-case method of this crate runs on (PLAN-RENDERER-WGPU-CRATES.md 7.2.3).", "//!",
         "//! Each field borrows one component of the composer, so a method touches only the components it names and borrow splitting stays field-precise. Methods of lower use-case crates are reached through the explicit `as_<crate>` bridges, which reborrow the same components.", "", "use super::*;", "", f"//#region {em}️{x}",
         f"/// {em}️ The borrowed component split of one `ShellState` for the `{crate}` use cases; `{x}::of` is the only constructor from the composer.", f"pub struct {x}<'a> {{"]
    for f, t in COMPONENTS: L.append(f"    pub {f}: &'a mut {t},")
    L += ["}", "", f"impl<'a> {x}<'a> {{", f"    /// {em}️ Splits the composer into its components.", "    pub fn of(shell: &'a mut ShellState) -> Self {", "        Self {"]
    for f, _ in COMPONENTS: L.append(f"            {f}: &mut shell.{f},")
    L += ["        }", "    }"]
    for lower in bridges:
        lx = cfg.CTX[lower]
        L += ["", f"    /// {em}️ The same components as a `{lx}`, for calling a use case of the lower crate `{lower}`.", f"    pub fn {BRIDGE[lower]}(&mut self) -> {lx}<'_> {{", f"        {lx} {{"]
        for f, _ in COMPONENTS: L.append(f"            {f}: &mut *self.{f},")
        L += ["        }", "    }"]
    L += ["}", f"//#endregion {em}️{x}", ""]
    return "\n".join(L)

def selfless_to_free(root, report):
    out = {}
    for crate in BRIDGE:
        for slug, p in crate_files(root, crate):
            doc = Doc(open(p, encoding="utf-8").read())
            while True:
                found = None
                for it in doc.items():
                    if it["kind"] == "impl" and re.match(r"impl\s+ShellState\b", it["head"]):
                        sl = []
                        for m in doc.members(it):
                            head = blank("\n".join(doc.lines[m["s"]:m["e"] + 1]))
                            if not re.search(r"\bself\b", head[:head.index("{")]): sl.append(m)
                        if sl: found = (it, sl); break
                if not found: break
                it, sl = found
                texts = []
                for m in sorted(sl, key=lambda m: -m["s"]):
                    assert doc.balanced(m["s"], m["e"]), ("region", m["name"])
                    t = doc.cut(m["s"], m["e"])
                    t = [l[4:] if l.startswith("    ") else l for l in t]
                    for k, l in enumerate(t):
                        mm = re.match(r"^((?:pub(?:\([^)]*\))?\s+)?)((?:async |const |unsafe )*fn )", l)
                        if mm:
                            if not mm.group(1): t[k] = "pub(super) " + l
                            break
                    texts.append(t); out[m["name"]] = crate
                it = next(i2 for i2 in doc.items() if i2["kind"] == "impl" and re.match(r"impl\s+ShellState\b", i2["head"]))
                for t in texts: doc.lines[it["e"] + 1:it["e"] + 1] = [""] + t
                doc.reload()
            open(p, "w", encoding="utf-8").write(doc.text())
    report.append(f"self-less associated functions turned free: {len(out)}")
    return out

def convert(root, report):
    selfless = selfless_to_free(root, report)
    ns = deps.load(root); deps.link(ns)
    homes = collections.defaultdict(set)
    for n in ns:
        if n.kind == "method": homes[n.name].add(n.crate)
    mnodes = [n for n in ns if n.kind == "method"]
    def head(n):
        b = blank(n.text); return b[:b.index("{")]
    refsig = {id(n) for n in mnodes if re.search(r"&\s*self\b", head(n)) and not re.search(r"&\s*mut\s+self\b", head(n))}
    need = set()
    for n in mnodes:
        if id(n) in refsig and any(t.kind == "method" and t.crate != n.crate for t in n.refs): need.add(n)
    ch = True
    while ch:
        ch = False
        for n in mnodes:
            if id(n) in refsig and n not in need and any(t in need for t in n.refs): need.add(n); ch = True
    needkey = {(n.path, n.name) for n in need}
    bridges = collections.defaultdict(set)
    manual = []
    for crate in BRIDGE:
        x = cfg.CTX[crate]
        for slug, p in crate_files(root, crate):
            t = open(p, encoding="utf-8").read(); doc = Doc(t)
            lines = doc.lines; offs = [0]
            for l in lines: offs.append(offs[-1] + len(l) + 1)
            b = "\n".join(doc.blank)
            edits = []
            for it in doc.items():
                if it["kind"] == "impl" and re.match(r"impl\s+ShellState\b", it["head"]):
                    k = it["s"]
                    while not lines[k].lstrip().startswith("impl"): k += 1
                    edits.append((offs[k], offs[k] + len(lines[k]), lines[k].replace("impl ShellState", f"impl<'ctx> {x}<'ctx>")))
                    for m in doc.members(it):
                        ms, me = offs[m["s"]], offs[m["e"] + 1] - 1
                        if (p, m["name"]) in needkey:
                            sg = re.search(r"&\s*self\b", b[ms:me])
                            edits.append((ms + sg.start(), ms + sg.end(), "&mut self"))
                        for mm in re.finditer(r"\bself\s*\.\s*([A-Za-z_][A-Za-z_0-9]*)(\s*::\s*<[^>]*>)?(\s*)\(", b[ms:me]):
                            name = mm.group(1)
                            if name not in homes: continue
                            hs = homes[name]; assert len(hs) == 1, (name, hs)
                            h = next(iter(hs))
                            if h == crate: continue
                            assert cfg.RANK[h] < cfg.RANK[crate], (m["name"], name, h, crate)
                            bridges[crate].add(h)
                            lp = ms + mm.end() - 1; rp = match_close(b, lp)
                            args = b[lp + 1:rp]
                            if re.search(r"\bself\b", args):
                                top = 0; commas = 0
                                for ch_ in args:
                                    if ch_ in "([{": top += 1
                                    elif ch_ in ")]}": top -= 1
                                    elif ch_ == "," and top == 0: commas += 1
                                if args.strip().endswith(",") and commas == 1: commas = 0
                                if commas == 0:
                                    edits.append((ms + mm.start(), rp + 1, "({ let __arg0 = " + t[lp + 1:rp] + "; self." + BRIDGE[h] + "()." + name + (mm.group(2) or "") + "(__arg0) })"))
                                    continue
                                manual.append((crate, m["name"], name)); report.append(f"MANUAL hoist {crate} {m['name']} -> {name}")
                            edits.append((ms + mm.start(), ms + mm.start() + 4, "self." + BRIDGE[h] + "()"))
            for s_, e_, r_ in sorted(edits, key=lambda e: (e[0], e[1]), reverse=True): t = t[:s_] + r_ + t[e_:]
            open(p, "w", encoding="utf-8").write(t)
    for crate in BRIDGE:
        if crate not in {n.crate for n in mnodes}: continue
        p = f"{cfg.stg(root, cfg.CRATE2DIR[crate])}/{BUNDLE_SLUG}/🦀️.rs"
        mover.write(p, bundle_text(crate, sorted(bridges[crate], key=lambda c: cfg.RANK[c])))
    report.append(f"upgraded to &mut self: {sorted((n.crate, n.name) for n in need)}")
    return selfless, manual
