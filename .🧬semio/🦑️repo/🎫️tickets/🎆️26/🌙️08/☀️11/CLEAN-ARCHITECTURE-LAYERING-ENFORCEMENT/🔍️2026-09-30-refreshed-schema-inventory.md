# Refreshed Schema Inventory

Executed root schema generation/docs successfully (14.6 seconds). Executed the global schema check (15.1 seconds): it remains failing, with 9362 concrete findings. This is not a passing full-tree check.

Selected owner findings (4):

- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧬️schema`: schema-owner-ineligible — ✏️s/🧑‍💻dev/🧹️fixture-sweep is not a declared schemaScopeOwnerLevels level.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🔣️.json`: schema-export-formats-annotation-invalid — DevContributionPathV1 declares x-semio-formats ["json"]; restricted support is a non-empty subset of 📜️wit, 🔗️graphql, 🔣️jsonschema, 🛰️protobuf, 🟦️typescript, 🦀️rust.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🔣️.json`: schema-export-id-invalid — Root title "Dev Contribution" is not a PascalCase export id.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🟦️.ts`: schema-export-incomplete — os.dev.contribution declares export DevContributionPathV1 in 🔣️.json; 🟦️typescript carries no declaration of it. Provide it, or annotate the export with x-semio-formats to declare restricted support honestly.

This snapshot ran while Cargo ownership schema relocation was in progress; the final inventory must refresh after all authored schema changes settle. Generated catalog/doc files are retained; process output is temporary ticket generated data.
