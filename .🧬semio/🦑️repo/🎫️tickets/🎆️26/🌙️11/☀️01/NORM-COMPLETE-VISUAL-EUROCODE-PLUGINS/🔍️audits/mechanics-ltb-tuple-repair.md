# Mechanics Ltb Sample Pair Repair

The TypeScript LtbMember twin contained invalid literal Rust fragments Vec<(f64 for moment, lineLoad and pointLoads. Its native schema defines each field as Vec<(f64, f64)>. The TypeScript fields now use [number, number][].

A focused JSON Schema contract defines the three required sample arrays, with each sample exactly two numbers. A language-neutral fixture supplies nine cases covering empty arrays, signed and fractional loads, missing fields, short/long pairs, text values, scalar input and object-shaped samples. The regression validates each case independently with Ajv and uses the actual TypeScript compiler to assign each wire vector to Pick<LtbMember, moment | lineLoad | pointLoads>, comparing both outcomes with the fixture. It also asserts the native Rust tuple declarations. No production parser or runtime library dependency was added.

Red: 0 passed / 1 failed; compiler reported Vec/f64 missing names and three missing closing parentheses. Green: 1 passed / 0 failed, 22 assertions. The main norm TypeScript runner fixture now includes this suite.

Command: repo --service=off tools exec --light bun -- test ./✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧪️tests/🏗️ltb-sample-pairs/🟦️.ts (session address and language environment supplied).

Changed files:

- ✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧬️schema/🏗️ltb-sample-pairs/🔣️.json
- ✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧪️tests/🏗️ltb-sample-pairs/🧫️cases.json
- ✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧪️tests/🏗️ltb-sample-pairs/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🏃️package-runner/🔣️.json
