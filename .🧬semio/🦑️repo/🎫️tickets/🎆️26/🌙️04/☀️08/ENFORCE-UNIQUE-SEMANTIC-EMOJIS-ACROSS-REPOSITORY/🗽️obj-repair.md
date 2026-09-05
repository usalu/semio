# OBJ Emoji Repair

The artifact root is `🗽️obj`, distinct from the sibling GLTF cube. Its subsets are `📐️geometry` and `🎨️material`. Mutation, suite, scenario, reader, fixture and carrier names were chosen individually for their roles; paired carriers use `⬅️before.obj` and `➡️after.obj`. Nested OBJ serializer directories outside this artifact retain their independently governed identities.

The existing generator now declares each recipe's output directory explicitly and writes into the correct owning subset. Source imports, descriptors, oracle catalogs and central overrides use the final paths. Three verified-empty duplicate texcoord directories were removed; populated implementations were preserved. No Git state was modified.

Verification:

- Geometry audit: 247 files, 149 directories, 385 governed entries, all eight categories zero. Material: 8 files, 16 directories, 24 governed entries, all categories zero. Physical inventory crosscheck included the ignored example OBJ carrier.
- Repository fixture verifier after the final artifact move: 31 fixtures, zero problems.
- Existing third-party `tobj` generator built and read the restored 487-byte pattern-shell fixture; the reader recipe and all ten `three` document recipes executed successfully through Nx.
- All 23 independently generated OBJ carriers matched their recorded SHA-256 digests and byte counts.
- The single catalog scenario directory resolves to its physical mutation test directory.

A reference edit briefly changed a comment inside pattern-shell. The exact original comment was restored after proving its original SHA-256; fixture data was not rebaselined. The retained `✳️geometry` text in that comment is immutable fixture data, not a live path reference.

The related OS event-page fixture names are now `🔗️event-page-bootstrap-v1.schema.json` and `🚀️event-page-bootstrap-v1.json`. The existing Nx bootstrap check executed all 11 checks successfully after exact reference repair.

Workspace completion remains pending the other trees and final global verification.
