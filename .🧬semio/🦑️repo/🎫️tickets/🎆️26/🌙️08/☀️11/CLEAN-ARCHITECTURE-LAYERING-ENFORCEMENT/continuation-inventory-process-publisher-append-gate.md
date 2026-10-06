# Inventory Process Publisher Append Gate

Exact publisher 5bf13a61667a34654c517548e7fcf12344ac2385028fd94cd846c0ae864a03f9 fixes the earlier journal truncation gap. One exclusive append FD retains complete JSON lines; each line uses full short-write loops and fsync before its associated source open/write. The first record embeds full plan and every record binds plan hash. Plan is durable before source writes; final/refused terminal is durable and the journal closes in finally.

Independent model/proposal/source joins, repeated current producer and all raw/input/provider guards, seven complete destination preimages/inverses, same-FD single-link/device/inode preimage controls and final source readbacks were reviewed. Current seven preimages, seven inputs and five providers still match. Publication admission is Ready; actual writes/journal must be observed separately. Atomic batch and ancestor-directory durability remain unclaimed.
