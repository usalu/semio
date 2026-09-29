"""🧯️ FH1 family A — every `.map_err(Fault::from)` over a store's `String` refusal names its literal framework code
(the census reads a path `Fault::from` as an undeclared raise)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
M = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
P = M + "🦀️.rs"


def raise_(code):
    return f'.map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("{code}"), reason))'


apply("", [
    (M + "👥️presence/♻️retirement/🦀️.rs", "            return match retirement.close_step(1, maximum_bytes).map_err(Fault::from)? {", "            return match retirement.close_step(1, maximum_bytes).map_err(|reason| Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new(\"plugin.presence.retirement-step\"), reason))? {"),
    (P, "                let presence_local = self.presence_store.local_read().map_err(Fault::from)?;", "                let presence_local = self.presence_store.local_read()" + raise_("plugin.presence.local-read") + "?;"),
    (P, "                        presence_local: Some(self.presence_store.local_read().map_err(Fault::from)?),", "                        presence_local: Some(self.presence_store.local_read()" + raise_("plugin.presence.local-read") + "?),"),
    (P, "                19 => self.presence_store.maintenance_local_reads_step(maximum_items.min(1), maximum_bytes).map_err(Fault::from).map(|step| match step {", "                19 => self.presence_store.maintenance_local_reads_step(maximum_items.min(1), maximum_bytes)" + raise_("plugin.presence.local-read-maintenance") + ".map(|step| match step {"),
    (P, "                return self.presence_store.maintenance_local_reads_step(1, maximum_bytes).map_err(Fault::from).map(|step| match step {", "                return self.presence_store.maintenance_local_reads_step(1, maximum_bytes)" + raise_("plugin.presence.local-read-maintenance") + ".map(|step| match step {"),
    (M + "🪟️window/🫧️transient/🦀️.rs", "        let snapshot = partition.store.current_read_erased().map_err(Fault::from)?;", "        let snapshot = partition.store.current_read_erased()" + raise_("window-transient.current-read") + "?;", 2),
    (M + "🪟️window/🫧️transient/🦀️.rs", "        match partition.store.maintenance_returned_reads_step(&self.owners.state_retirement, maximum_items.min(1), maximum_bytes).map_err(Fault::from)? {", "        match partition.store.maintenance_returned_reads_step(&self.owners.state_retirement, maximum_items.min(1), maximum_bytes)" + raise_("window-transient.read-maintenance") + "? {"),
    (M + "🫧️transient/🧵️publication/🦀️.rs", "        match retirement.close_step(maximum_items, maximum_bytes).map_err(Fault::from)? {", "        match retirement.close_step(maximum_items, maximum_bytes)" + raise_("transient.retirement-step") + "? {"),
])
