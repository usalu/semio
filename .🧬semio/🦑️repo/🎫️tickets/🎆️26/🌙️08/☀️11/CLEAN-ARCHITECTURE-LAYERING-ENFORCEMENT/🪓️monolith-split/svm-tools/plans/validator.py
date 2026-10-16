PLAN = dict(
    name="validator",
    orig="svm-tools/orig/validator.rs",
    real="🧰️framework/🔨️modules/🧬️schema/✅️validator/🦀️.rs",
    children=[
        dict(key="compile", folder="🔨️compile", mod="compile", doc="🔨️ Schema compilation: `$ref` scopes, keyword checks and fragment-context validation.", ranges=[(384, 898)]),
        dict(key="validate", folder="🔎️validate", mod="validate", doc="🔎️ Instance validation of compiled schemas: objects, arrays, strings and numbers.", ranges=[(900, 1199)]),
        dict(key="string_format", folder="📃️string-format", mod="string_format", doc="📃️ The pinned JSON Schema `format` policy: seven asserted string formats.", ranges=[(1201, 1407)]),
        dict(key="pattern", folder="🔤️pattern", mod="pattern", doc="🔤️ Owned backtracking matcher for the ECMA-262 `pattern` subset.", ranges=[(1409, 1753)]),
    ],
)
PLAN["real_lib"] = "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/🦀️.rs"
PLAN["oracle"] = dict(trees=["🧰️framework/🔨️modules/🧬️schema/✅️validator"], crate="semio_framework_schema_validator", cfgs=["lib", "test"])
