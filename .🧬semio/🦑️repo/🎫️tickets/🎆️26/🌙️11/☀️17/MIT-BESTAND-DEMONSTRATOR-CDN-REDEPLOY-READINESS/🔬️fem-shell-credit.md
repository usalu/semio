# FEM shell credit on Wasm32

The FEM build wbXjJ9G failed in the 2D session registry during its in-lane retry. Multiplying 64 shells by the per-shell limit of 1,507,885,056 bytes overflows a 32-bit address. No FEM component linked.

The registry already requires an empty shell, checks each request against both per-shell maxima, and checks both actual aggregate additions for integer overflow before mutating any credit. Together these rules enforce the physical target limit and the mathematical per-shell total. Removing the redundant multiplication allows the Wasm compile without raising either limit.

The four-shell fixture exercises two maximum-sized byte reservations on a 32-bit target. The third and fourth are refused without consuming credit. At 64 bits all four fit. Both targets can release and reuse a prior reservation. The extracted production methods reproduced the original Rust compile error before the change, then agreed with independent SQLite arithmetic at each width. A fresh complete FEM queue build and Statik boot are pending.
