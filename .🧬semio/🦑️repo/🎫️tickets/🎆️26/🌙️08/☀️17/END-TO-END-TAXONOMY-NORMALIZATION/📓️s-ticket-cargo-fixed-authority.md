# Ticket Cargo Fixed Authority

## Boundary

Three exact Cargo-owned fixed-filename contracts close the governed residual without creating repository-wide wildcard authority:

- `ticket-cargo-manifest` admits `Cargo.toml` only beneath the complete canonical or embedded ticket hierarchy;
- `ticket-cargo-lock` admits `Cargo.lock` only beneath that same hierarchy;
- `root-cargo-lock` admits only the repository-root `Cargo.lock`.

The ticket patterns freeze the metadata root plus two-digit year, month, and day segments. A production package, malformed date segment, or nested workspace manifest therefore remains outside these contracts. The three separately classified nested workspace Cargo files still require physical taxonomy decisions.

## TDD evidence

The new permanent test first failed because none of the three contract identities existed. The green run proves exact positive and negative vectors, strict shipped-schema validation, and third-party Cargo parity by creating an isolated governed ticket package and successfully running `cargo metadata --no-deps --format-version 1` against it.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-cargo-fixed-authority.test.ts'
2 pass
0 fail
12 expect() calls
```

Direct shipped-schema validation returned `problems=[]` with 60 fixed-filename contracts. The canonical recursively byte-key-sorted taxonomy SHA-256 at this checkpoint is `798455be7dbfcc404e2602ea7e01781e08834836c278b17fd075b2264ca3f9a7`; the source JSON byte SHA-256 is `f7de61177a71c4c6f897034dee09b3080bd1a3ffc43a830bd62189967a7515a0`.

The combined permanent schema gate covering standard/subset parents, projected-profile inference, Cargo cache scope, CLI artifact directory kinds, and the new Cargo contracts also completed green:

```text
13 pass
0 fail
73 expect() calls
5 files
6.27 seconds
```

The broader library `loadTaxonomy|validateTaxonomy` selector completed with `43 pass`, `196 filtered out`, `0 fail`, and `222 expect()` calls in 5.08 seconds.

The updated discovery/schema boundary also bundled successfully for Bun as one module (`211.22 KB`, 100 ms) into retained ticket evidence `🧪️discovery-schema-build.js`.
