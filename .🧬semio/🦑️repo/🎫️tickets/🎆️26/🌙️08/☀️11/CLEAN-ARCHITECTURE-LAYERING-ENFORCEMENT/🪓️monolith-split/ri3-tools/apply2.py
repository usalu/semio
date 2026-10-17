"""🧪 RI3 target 2: `HistoryState.host_route_folder` and `HostRouteFolder` are ungated in shell-history (the one crate that defines them), so no other crate's `testkit` or `cfg(test)` can decide whether the field exists; the shell-state initialiser loses its gate."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from common_ri3 import *
from edit import apply

G = '#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]\n'
apply([
    (f"{STG}/⏪️history/🧬️component/🦀️.rs", "    " + G + "    pub host_route_folder:", "    pub host_route_folder:", 1),
    (f"{STG}/⏪️history/📎️local-folders/🦀️.rs", G + "#[derive(Clone, Debug, PartialEq, Eq)]\npub struct HostRouteFolder", "#[derive(Clone, Debug, PartialEq, Eq)]\npub struct HostRouteFolder", 1),
    (f"{STG}/🐙️state/🧩️session-and-topology/🦀️.rs", "                " + G + "                host_route_folder: None,", "                host_route_folder: None,", 1),
])
