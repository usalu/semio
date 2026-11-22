# Release headroom recheck

Host and mounted Rust image initially reported 6.6 GiB available. During this read-only check both rebounded to approximately 13 GiB without a cleanup action. The active Process and Demonstrator build lanes `p20812e` and `pbd42c9` measured about 9.77 and 6.85 GiB allocated respectively and remain excluded. The browser module and component staging trees contain release inputs and were not selected for reclamation.

The targeted supported `repo cache gc --dry-run --area=retired,cargo-store,unit-cache` reported zero removable entries and zero bytes. The ticket-owned completed glTF graph scratch has 525,280 KiB allocated under `🗑️generated/gltf-graph/home/registry` (about 513 MiB); no corresponding command or open file was observed in the bounded process and `lsof +D` checks. It is diagnostic registry material, not an accepted release asset. APFS sharing can make its actual host free-space gain smaller than `du`, and no deletion was performed. Other large ticket scratch belongs to the retained Demonstrator or Procedural investigations and remains excluded.

Old Energy run w79IQpU has 1,006,676 KiB allocated in its mounted-image work directory, but is retained diagnostic evidence and its image-level allocation does not prove immediate host free-space recovery. It was not proposed for removal. No substantial supported idle reclaim with a proven host gain was identified; no mounts, processes, release inputs or disk state were changed.

At 12:36 Pacific, the Procedural lane was waiting for compiler-image headroom. Removed the completed ticket-generated gltf-graph/home/registry after a fresh open-file check. Recorded core hashes not applicable. Host free-space change during removal: 523 MiB.

At 12:36 Pacific, the Procedural lane was waiting for compiler-image headroom. Removed the completed ticket-generated probe-wakSUm7-40026 after a fresh open-file check. Recorded core hashes [('semio_hub_demonstrator_component.core.wasm', '409c9a4f64cb03da46be149ec275698dff6cad7280d7b7aafa23fe569f289978')]. Host free-space change during removal: 347 MiB.

At 12:36 Pacific, the Procedural lane was waiting for compiler-image headroom. Removed the completed ticket-generated probe-wKD5kO3-45521 after a fresh open-file check. Recorded core hashes [('semio_hub_procedural_component.core.wasm', 'aef9d8c6b2f7a941be6ffd44187542251d7a16a26eae8a7d8bae9286af761053')]. Host free-space change during removal: 145 MiB.

At 12:36 Pacific, the Procedural lane was waiting for compiler-image headroom. Removed the completed ticket-generated probe-wqks1dx-65558 after a fresh open-file check. Recorded core hashes [('semio_hub_procedural_component.core.wasm', '0947aa92ecc63afa8ba013f8b8021f032d09247dc9ea48cf3a6c3100a3aa8c5b')]. Host free-space change during removal: 143 MiB.
