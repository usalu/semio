# Fixture Reproduction Output Path

## Change

`fixture reproduce` and `fixture generate` now resolve generated files at
`<SEMIO_FIXTURE_OUT>/<fixture-id>/<filename>`. Recorded generators use the
fixture id as their recipe directory, so the previous shallow lookup could not
find their output.

## Verification

Executed from the repository root:

```sh
bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture reproduce --subset cc6
```

Result:

```text
[fixture reproduce] 119 generated fixture(s), 0 problem(s)
```

`fixture generate` received the identical path correction but was not executed,
because it installs generated content into the fixture store.
