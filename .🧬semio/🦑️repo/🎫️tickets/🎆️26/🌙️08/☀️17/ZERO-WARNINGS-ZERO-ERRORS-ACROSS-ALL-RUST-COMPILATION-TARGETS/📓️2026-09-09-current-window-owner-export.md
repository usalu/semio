# Current Window Owner Export

wasm1027 reported one unresolved bounded_window_transient_store_owners import. A fresh source read found that symbol already absent from the current plugin exports and window module. No source edit was needed. The current window API exports WindowTransientOwnerBundle and concrete owners. A new strict pass is required because the failing diagnostic describes an earlier source snapshot.
