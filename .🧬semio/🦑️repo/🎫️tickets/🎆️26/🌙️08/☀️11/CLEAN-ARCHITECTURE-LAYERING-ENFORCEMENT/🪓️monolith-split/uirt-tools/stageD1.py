"""🚧️ Stage D1: moves the resident-credit helpers next to the limits they price (primitives)."""
import re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
P=f"{REPO}/{RT}/🧱️primitives"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/snapA"
snap=open(f"{S}/core.rs",encoding="utf-8").read().split("\n")
def L(a,b): return "\n".join(snap[a-1:b])
assert "What one reconcile reservation costs" in snap[1390-1] and snap[1401-1].startswith("const _: () = assert!(SURFACE_RECONCILE_FLOOR_ITEMS")
floor=L(1390,1401)
rel=L(1420,1431)
assert rel.startswith("fn release_surface_reconcile(") and "fn release_surface_reconcile_one" in rel
rel=rel.replace("fn release_surface_reconcile(","pub fn release_surface_reconcile(").replace("fn release_surface_reconcile_one(","pub fn release_surface_reconcile_one(")
add=floor+'''

pub fn surface_reconcile_limits_admissible(limits: SurfaceReconcileLimits) -> bool {
    limits.max_nodes <= SurfaceReconcileLimits::default().max_nodes
        && limits.max_items <= SurfaceReconcileLimits::default().max_items
        && limits.max_bytes <= SurfaceReconcileLimits::default().max_bytes
        && limits.max_identifier_bytes <= SurfaceReconcileLimits::default().max_identifier_bytes
}

pub fn reserve_surface_reconcile_floor(limits: SurfaceReconcileLimits) -> Option<ui_contract::UiResidentPermit> {
    let mut permit = None;
    let floor = ui_contract::UiResidentLimits { items: SURFACE_RECONCILE_FLOOR_ITEMS.min(limits.max_items), bytes: SURFACE_RECONCILE_FLOOR_BYTES.min(limits.max_bytes) };
    ui_contract::UiResidentPermit::try_reserve(floor, &mut permit, SURFACE_RECONCILE_PAGE_BYTES).ok()?;
    permit
}

'''+rel
p=f"{P}/🚧️limits/🦀️.rs"
t=open(p,encoding="utf-8").read()
anchor="pub const SURFACE_RECONCILE_SURFACE_BYTES: usize = ui_contract::UI_RESIDENT_SURFACE_BYTES;\n"
assert t.count(anchor)==1 and "SURFACE_RECONCILE_FLOOR_BYTES" not in t
t=t.replace(anchor,anchor+"\n"+add+"\n")
open(p,"w",encoding="utf-8").write(t)
# root re-exports
p=f"{P}/🦀️.rs"
t=open(p,encoding="utf-8").read()
old="pub use limits::{SURFACE_COMPONENT_COPY_WORK_BYTES, "
assert t.count(old)==1
t=t.replace(old,"pub use limits::{SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_FLOOR_BYTES, SURFACE_RECONCILE_FLOOR_ITEMS, release_surface_reconcile, release_surface_reconcile_one, reserve_surface_reconcile_floor, surface_reconcile_limits_admissible, ")
open(p,"w",encoding="utf-8").write(t)
print("ok")
