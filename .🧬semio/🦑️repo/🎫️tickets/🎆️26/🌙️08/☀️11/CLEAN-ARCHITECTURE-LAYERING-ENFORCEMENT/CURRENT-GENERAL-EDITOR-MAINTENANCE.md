# General Editor Maintenance

Root removed the unused OS Kernel alias from the private Editor color conversion module. Both conversion bodies already name the defining General Value traits and types directly, so the alias supplied no executable symbol.

Root also removed the historical comment claiming that Value derive defaults to OS Kernel. The current defining expansion uses `::semio_framework_value` when no crate override is supplied, as directly inspected in `value/derive/expansion`. This is source and documentation cleanup; no new runtime feature or passing native test is claimed.

The Editor package still declares normal Kernel and Infinite dependencies, and its mounted binary/schema/error roles still require the larger coherent rendering cut. This cleanup does not prove their physical deletion or public API closure.
