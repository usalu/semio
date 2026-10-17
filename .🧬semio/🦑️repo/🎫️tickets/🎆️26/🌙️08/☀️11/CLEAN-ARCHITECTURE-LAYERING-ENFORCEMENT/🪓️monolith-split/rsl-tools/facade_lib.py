"""📚️ Shared helpers of the F-04(a) tools: crate table, skip zones, single-line dependency entry reader for Cargo manifests."""
import os, re
PKG={"abi":"semio-framework-abi","action_bus":"semio-framework-action-bus","interaction":"semio-framework-interaction","onboarding":"semio-framework-onboarding","history_edit":"semio-framework-time-travel-actions","tool_run":"semio-framework-tool-run-actions","platform":"semio-framework-action-bus","io_schema":"semio-framework-io-schema","sqlite_snapshot":"semio-framework-io-sqlite-snapshot","mesh_io":"semio-framework-mesh-engine"}
DIR={"semio-framework-abi":"🧰️framework/🔨️modules/🌉️abi","semio-framework-action-bus":"🧰️framework/🔨️modules/🎯️action-bus","semio-framework-interaction":"🧰️framework/🔨️modules/🕹️interaction","semio-framework-onboarding":"🧰️framework/🔨️modules/🎓️onboarding","semio-framework-time-travel-actions":"🧰️framework/🔨️modules/⏪️time-travel/🕹️actions","semio-framework-tool-run-actions":"🧰️framework/🔨️modules/⏯️tool-run/🕹️actions","semio-framework-io-schema":"🧰️framework/🔨️modules/🚪️io/🧬️schema","semio-framework-io-sqlite-snapshot":"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot","semio-framework-mesh-engine":"🧰️framework/🔨️modules/🏗️mesh-engine"}
PKGDIR={k:v+"/📦️packages/🦀️rust" for k,v in DIR.items()}
FACADE_MANIFEST="🧰️framework/📦️packages/🦀️rust/Cargo.toml"
SKIP_SEGMENTS=("/🔌️plugin/","/📺️renderer/")
def in_skip_zone(path):
    if os.environ.get("RSL_ZONE")=="all": return False
    return any(s in path for s in SKIP_SEGMENTS)
SEC=re.compile(r'^\[([^\]]+)\]\s*$')
ENT=re.compile(r'^(\s*)([A-Za-z0-9_\-]+)(\.workspace)?\s*=\s*(.*?)\s*$')
def dep_entries(text):
    """single-line entries of dependency tables: dict(section,key,pkg,style,line,indent)"""
    out=[]; sec=None
    for i,l in enumerate(text.split("\n")):
        m=SEC.match(l)
        if m: sec=m.group(1); continue
        if sec is None or not re.search(r'(^|\.)(dependencies|dev-dependencies|build-dependencies)$',sec): continue
        m=ENT.match(l)
        if not m: continue
        key=m.group(2); val=m.group(4)
        if m.group(3): style="workspace-dot"; pkg=key
        elif "workspace" in val and re.search(r'workspace\s*=\s*true',val): style="workspace"; pkg=key
        elif "path" in val: style="path"; pkg=key
        else: style="other"; pkg=key
        pm=re.search(r'package\s*=\s*"([^"]+)"',val)
        if pm: pkg=pm.group(1)
        out.append(dict(section=sec,key=key,pkg=pkg,style=style,line=i,indent=m.group(1),val=val))
    return out
