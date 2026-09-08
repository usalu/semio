# Space Host Export Names

Pass555 replaces the two artifact glob exports in the Space host module with explicit domain exports. Both artifact crates define package_descriptor, which native532 reported as an ambiguous glob export. The host now exposes these factories as collection_package_descriptor and space_package_descriptor, alongside the existing distinct domain types, helpers and schema constants. Each artifact crate retains its own package_descriptor API.

Changed: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs`.

Source declarations were inspected in both artifact roots. Fresh syntax and compiler validation remain required.

