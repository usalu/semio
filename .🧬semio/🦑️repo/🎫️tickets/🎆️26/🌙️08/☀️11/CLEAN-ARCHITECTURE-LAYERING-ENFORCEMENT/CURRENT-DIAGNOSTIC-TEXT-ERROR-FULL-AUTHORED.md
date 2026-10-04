# Diagnostic TextError Authored Proposals

Complete ticket-only inputs. These captures do not imply native compiler/runtime admission. Originals and literal inverses: [full before](CURRENT-DIAGNOSTIC-TEXT-ERROR-FULL-BEFORE.md).

## native-tests.rs

5835 bytes; SHA-256 1b8d56d5ead94d27a7b5b292615f78dc1e3be5f955c7c1dcec89b9791125e6b6

```
use super::*;
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, ToValue, ValueError, ValueRefusalKind};
use std::{io::Write, process::{Command, Stdio}};

fn text_refusal_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn text_refusal_kind(value: &str) -> ValueRefusalKind {
    match value { "invalidValue" => ValueRefusalKind::InvalidValue, "canceled" => ValueRefusalKind::Canceled, "ownershipLimit" => ValueRefusalKind::OwnershipLimit, "allocationFailed" => ValueRefusalKind::AllocationFailed, "workLimit" => ValueRefusalKind::WorkLimit, "depthLimit" => ValueRefusalKind::DepthLimit, "unsupportedOwner" => ValueRefusalKind::UnsupportedOwner, "invariantViolated" => ValueRefusalKind::InvariantViolated, _ => panic!("closed kind") }
}
fn text_refusal_span(value: &serde_json::Value) -> TextSpan { TextSpan { line: u32::try_from(value["line"].as_u64().unwrap()).unwrap(), column: u32::try_from(value["column"].as_u64().unwrap()).unwrap(), length: u32::try_from(value["length"].as_u64().unwrap()).unwrap() } }

#[test]
fn controlled_diagnostic_text_error_retains_explicit_kind_and_span_corpus() {
    let fixture = text_refusal_fixture();
    let mut outputs = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let kind = text_refusal_kind(row["kind"].as_str().unwrap()); let span = text_refusal_span(&row["span"]); let message = row["message"].as_str().unwrap();
        let error = match row["operation"].as_str().unwrap() {
            "new" => TextError::new(kind, message, span),
            "expected" => TextError::expected(kind, message, span, row["expectedWire"]["expected"].as_str().unwrap()),
            "fromValueError" => TextError::from_value_error(ValueError::new(kind, message), span),
            "fromDiagnostic" => { let mut diagnostic = Diagnostic::error("canceled", span, message).with_expected(ExpectedSet { tokens: vec!["=".into()], keywords: vec!["name".into()], keys: vec!["type".into()] }); diagnostic.severity = Severity::Fatal; assert_eq!(TextError::from_diagnostic(kind, diagnostic.clone()), diagnostic.clone().into_text_error(kind)); diagnostic.into_text_error(kind) },
            _ => panic!("closed operation"),
        };
        assert_eq!(error.kind, kind, "{}", row["id"]); assert_eq!(error.span, span); assert_eq!(error.to_string(), row["expectedDisplay"].as_str().unwrap());
        let mut accept = |_| true; let mut encode = NativeEncodeControl::new(1_000_000, &mut accept);
        let wire = error.to_value_controlled(&mut encode).unwrap(); let projected = serde_json::Value::from(&wire);
        assert_eq!(projected, row["expectedWire"], "{}", row["id"]);
        let mut accept = |_| true; assert_eq!(TextError::from_value_controlled(&wire, &mut NativeDecodeControl::new(1_000_000, &mut accept)).unwrap(), error);
        outputs.push(serde_json::json!({"wire":projected,"display":error.to_string()}));
    }
    for row in fixture["wireCases"].as_array().unwrap() {
        let input = DslValue::from(&row["input"]); let mut accept = |_| true;
        let error = TextError::from_value_controlled(&input, &mut NativeDecodeControl::new(1_000_000, &mut accept)).unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue, "{}", row["id"]);
    }
    let limits = Limits { max_bytes: 0, max_tokens: 0, max_depth: 0, max_nodes: 0 };
    for row in fixture["limits"].as_array().unwrap() {
        let source_span = TextSpan::at(7, 3);
        let error = match row["operation"].as_str().unwrap() { "bytes" => limits.check_bytes(1), "depth" => limits.check_depth(1, source_span), "tokens" => limits.check_tokens(1, source_span), "nodes" => limits.check_nodes(1, source_span), _ => panic!("closed limit") }.unwrap_err();
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap()); assert_eq!(error.span, text_refusal_span(&row["expectedSpan"]));
    }
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let diagnostic_schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🎛️controlled/🔣️.json")).unwrap();
    let value_schema: serde_json::Value = serde_json::from_str(include_str!("../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json")).unwrap();
    let script = "import Ajv from'ajv/dist/2020.js';import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const a=new Ajv({strict:true}).addSchema(x.valueSchema).addSchema(x.diagnosticSchema);if(!a.validate(x.schema,x.fixture))throw Error(JSON.stringify(a.errors));const v=a.getSchema(x.diagnosticSchema.$id+'#/$defs/TextError');for(const r of x.fixture.wireCases)if(v(r.input)!==r.accepted)throw Error(r.id);const db=new Database(':memory:');const q=db.query(\"SELECT ? || ' at ' || ? || ':' || ? AS display\");const output=x.fixture.cases.map(r=>({wire:r.expectedWire,display:q.get(r.message,r.span.line,r.span.column).display}));db.close();await Bun.write(Bun.stdout,JSON.stringify(output));";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"diagnosticSchema":diagnostic_schema,"valueSchema":value_schema,"fixture":fixture}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr)); assert_eq!(serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap(), outputs);
    println!("[DEBUG] TextError preserves explicit eight-kind authority and actual spans through constructors, Diagnostic conversion and controlled wire against Ajv and SQLite");
}

```

## span.ts

185 bytes; SHA-256 6568b9a4ef47422e46a6490d102dfa84792609f067817258af58fc56e1cc68c4

```
/** 📍️ The owned diagnostic source position declared by its closed schema. */
export interface TextSpan { readonly line: number; readonly column: number; readonly length: number }

```

## diagnostic-controlled-schema.json

9122 bytes; SHA-256 48ca2667fae615c988bacd7d90af0ae8f258be5ef2c8154b8895c40e330735b5

```
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://semio.tech/schema/diagnostic/controlled",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "cases",
    "stress",
    "duplicateCases",
    "refusalKinds"
  ],
  "properties": {
    "cases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "id": {
            "type": "string"
          },
          "owner": {
            "enum": [
              "FaultCode",
              "Severity",
              "FaultOrigin",
              "TextSpan",
              "FaultScope",
              "ExpectedSet",
              "FaultCause",
              "TextError",
              "Diagnostic",
              "Fault"
            ]
          },
          "input": {},
          "expected": {},
          "accepted": {
            "type": "boolean"
          }
        },
        "required": [
          "id",
          "owner",
          "input",
          "accepted"
        ]
      }
    },
    "stress": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "listCount": {
          "type": "integer",
          "minimum": 256
        },
        "textRepetitions": {
          "type": "integer",
          "minimum": 65536
        },
        "text": {
          "type": "string"
        },
        "maximumBytes": {
          "type": "integer",
          "minimum": 1
        },
        "cancelAt": {
          "type": "integer",
          "minimum": 1
        }
      },
      "required": [
        "listCount",
        "textRepetitions",
        "text",
        "maximumBytes",
        "cancelAt"
      ]
    },
    "duplicateCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "owner",
          "entries"
        ],
        "properties": {
          "id": {
            "type": "string"
          },
          "owner": {
            "enum": [
              "FaultCause",
              "FaultScope",
              "TextSpan"
            ]
          },
          "entries": {
            "type": "array",
            "minItems": 2,
            "items": {
              "type": "array",
              "prefixItems": [
                {
                  "type": "string"
                },
                {}
              ],
              "items": false,
              "minItems": 2,
              "maxItems": 2
            }
          }
        }
      }
    },
    "refusalKinds": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "malformed",
        "ownership",
        "cancel"
      ],
      "properties": {
        "malformed": {
          "const": "invalidValue"
        },
        "ownership": {
          "const": "ownershipLimit"
        },
        "cancel": {
          "const": "canceled"
        }
      }
    }
  },
  "$defs": {
    "FaultCode": {
      "type": "string"
    },
    "Severity": {
      "enum": [
        "info",
        "warning",
        "error",
        "fatal"
      ]
    },
    "FaultOrigin": {
      "enum": [
        "edge",
        "renderer",
        "os",
        "module",
        "plugin",
        "app",
        "extension",
        "framework"
      ]
    },
    "TextSpan": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "line": {
          "type": "integer",
          "minimum": 0,
          "maximum": 4294967295
        },
        "column": {
          "type": "integer",
          "minimum": 0,
          "maximum": 4294967295
        },
        "length": {
          "type": "integer",
          "minimum": 0,
          "maximum": 4294967295
        }
      },
      "required": [
        "line",
        "column",
        "length"
      ]
    },
    "FaultScope": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "pluginId": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "appId": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "instanceId": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "module": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "bodyKey": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": []
    },
    "ExpectedSet": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "tokens": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "keywords": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "keys": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": []
    },
    "FaultCause": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "message": {
          "type": "string"
        },
        "code": {
          "anyOf": [
            {
              "$ref": "#/$defs/FaultCode"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "message"
      ]
    },
    "TextError": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "message": {
          "type": "string"
        },
        "span": {
          "$ref": "#/$defs/TextSpan"
        },
        "expected": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "kind": {
          "$ref": "https://semio.tech/schema/value/refusal#/$defs/kind"
        }
      },
      "required": [
        "kind",
        "message",
        "span"
      ]
    },
    "Diagnostic": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "code": {
          "$ref": "#/$defs/FaultCode"
        },
        "severity": {
          "$ref": "#/$defs/Severity"
        },
        "span": {
          "$ref": "#/$defs/TextSpan"
        },
        "message": {
          "type": "string"
        },
        "expected": {
          "anyOf": [
            {
              "$ref": "#/$defs/ExpectedSet"
            },
            {
              "type": "null"
            }
          ]
        },
        "scope": {
          "anyOf": [
            {
              "$ref": "#/$defs/FaultScope"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "code",
        "severity",
        "span",
        "message"
      ]
    },
    "Fault": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "origin": {
          "$ref": "#/$defs/FaultOrigin"
        },
        "code": {
          "$ref": "#/$defs/FaultCode"
        },
        "severity": {
          "$ref": "#/$defs/Severity"
        },
        "message": {
          "type": "string"
        },
        "scope": {
          "anyOf": [
            {
              "$ref": "#/$defs/FaultScope"
            },
            {
              "type": "null"
            }
          ]
        },
        "span": {
          "anyOf": [
            {
              "$ref": "#/$defs/TextSpan"
            },
            {
              "type": "null"
            }
          ]
        },
        "causes": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "$ref": "#/$defs/FaultCause"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "retryable": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "origin",
        "code",
        "severity",
        "message"
      ]
    }
  }
}

```

## retirement-production.rs

1264 bytes; SHA-256 bf3965619f671fe3bd8a772f20994e22fdd78a70b29fe90394e2183a9dbd7b94

```
semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
impl semio_framework_value::retirement::RetireOwned for crate::TextError {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { kind: _, message, span, expected } = self;
        semio_framework_value::artifact_retirement_sequence![message, span, expected]
    }
}
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
semio_framework_value::artifact_retire_struct!(crate::FaultScope { plugin_id, app_id, instance_id, module, body_key });
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
semio_framework_value::artifact_retire_struct!(crate::Fault { origin, code, severity, message, scope, span, causes, retryable });

```

## launch-entry.json

541 bytes; SHA-256 627d4c6c2c69b38fcd720d46ffd34400376f7c744187bb80c2bf3708cbac3755

```
{
  "name": "⚖️test-text-error-portable⚠️diagnostic🦀️",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run @semio-tech/framework-diagnostic-rs:test-text-error-portable --skip-nx-cache",
  "cwd": "${workspaceFolder}",
  "env": {
    "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/value-refusal/portable"
  },
  "presentation": {
    "group": "4_gate",
    "order": 900.01781
  }
}

```

## portable-tests.ts

2702 bytes; SHA-256 e6c6b139cb6d72b316b54b725a6b3ae7229351aae28ac5be3c25f69d5fe0933c

```
import assert from "node:assert/strict";
import { test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import { TextError } from "../🟦️.ts";
import { ValueError, type ValueRefusalKind } from "../../../🌱️value/⚠️refusal/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import diagnosticSchema from "../../🧬️schema/🎛️controlled/🔣️.json" with { type: "json" };
import valueSchema from "../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json" with { type: "json" };

test("owned TextError scalar construction preserves schema kind wire and display against Ajv and SQLite", () => {
  const ajv = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(diagnosticSchema); assert(ajv.validate(schema, fixture), JSON.stringify(ajv.errors));
  const validate = ajv.getSchema(`${diagnosticSchema.$id}#/$defs/TextError`); assert(validate);
  for (const row of fixture.wireCases) assert.equal(validate(row.input), row.accepted, row.id);
  const db = new Database(":memory:"); const query = db.query("SELECT ? || ' at ' || ? || ':' || ? AS display");
  try {
    for (const row of fixture.cases) {
      const expected = "expected" in row.expectedWire ? row.expectedWire.expected : undefined;
      const error = expected === undefined ? new TextError(row.kind as ValueRefusalKind, row.message, row.span) : TextError.expected(row.kind as ValueRefusalKind, row.message, row.span, expected);
      const reference = query.get(row.message, row.span.line, row.span.column) as { display: string };
      assert.equal(error.kind, row.kind, row.id); assert.deepEqual(error.toWire(), row.expectedWire, row.id); assert.equal(error.toString(), reference.display, row.id); assert.equal(error.toString(), row.expectedDisplay, row.id);
    }
  } finally { db.close(); }
});
test("actual TextError source boundary preserves original owned ValueError kind and dotted message", () => {
  for (const row of fixture.cases.filter(row => row.operation === "fromValueError")) {
    const sourceSpan = { ...row.span };
    const error = TextError.fromValueError(new ValueError(row.kind as ValueRefusalKind, row.message), sourceSpan);
    sourceSpan.line = 99;
    assert.equal(error.kind, row.kind); assert.equal(error.message, row.message); assert.deepEqual(error.span, row.span); assert.deepEqual(error.toWire(), row.expectedWire); assert.equal(error.toString(), row.expectedDisplay);
  }
  console.log("[DEBUG] TextError actual portable boundary retains eight kinds, exact span and dotted message without prose classification");
});

```

## root-test-only.rs

32419 bytes; SHA-256 a8697bbbfb27b9a8238add77ebfe95430e644a6bd358fdcbfde6edd7b73b8a8c

```
//! ⚠️ Text errors, structured diagnostics, fault reporting, and parse limits.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

pub use crate::span::TextSpan;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
#[path = "🎛️controlled/🦀️.rs"]
pub(crate) mod controlled;

//#region 🔖️Errors
/// 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq)]
pub struct TextError {
    pub message: String,
    pub span: TextSpan,
    pub expected: Option<String>,
}

impl std::fmt::Display for TextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}:{}", self.message, self.span.line, self.span.column)
    }
}

impl std::error::Error for TextError {}

/// 🌉️ Hand-written, not derived — same DAG reason as `FaultCode`/`Severity` above. No
/// `#[serde(rename_all = …)]` on the original, so field names stay as declared.
impl ToValue for TextError {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_text_error(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone())), ("span".to_string(), self.span.to_value())];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), DslValue::String(expected.clone())));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for TextError {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_text_error(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TextError, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.message, found {other:?}"))),
        };
        let span = match find("span") {
            Some(slot) => TextSpan::from_value(slot)?,
            None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing TextError.span")),
        };
        let expected = match find("expected") {
            None | Some(DslValue::Null) => None,
            Some(DslValue::String(text)) => Some(text),
            Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.expected, found {other:?}"))),
        };
        Ok(TextError { message, span, expected })
    }
}

impl TextError {
    pub fn new(message: impl Into<String>, span: TextSpan) -> Self {
        Self { message: message.into(), span, expected: None }
    }

    pub fn expected(message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { message: message.into(), span, expected: Some(expected.into()) }
    }

    pub fn from_diagnostic(diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error()
    }
}

/// 🏷️ Stable dotted fault/diagnostic code (e.g. `module.pack.checksum-mismatch`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaultCode(pub String);

impl FaultCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<&'static str> for FaultCode {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

/// 🌉️ Hand-written, not derived: `FaultCode` is `#[serde(transparent)]`, a shape
/// `#[derive(ToValue, FromValue)]` does not support (see its own module docstring).
impl ToValue for FaultCode {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_code(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.clone())
    }
}
impl FromValue for FaultCode {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_code(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(FaultCode(s)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCode, found {other:?}"))),
        }
    }
}

/// 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

impl From<String> for FaultCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DiagnosticCode> for FaultCode {
    fn from(value: DiagnosticCode) -> Self {
        FaultCode::new(value.0)
    }
}

/// 🚦️ Declaration order is the level order: `#[derive(PartialOrd, Ord)]` makes `Info < Warning <
/// Error < Fatal` a structural fact, not a hand-maintained comparator. `as_u8`/`from_u8` (0..3)
/// give a stable wire-compatible numeric mirror for TS/WIT twins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🌉️ Hand-written, not derived: `Severity` is a plain unit-only "string enum" (`#[serde(rename_all
/// = "camelCase")]` with no `tag`, serde's default bare-string representation) —
/// `#[derive(ToValue, FromValue)]`'s enum path only supports internally-tagged representations.
impl ToValue for Severity {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_severity(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for Severity {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_severity(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "info" => Ok(Severity::Info),
                "warning" => Ok(Severity::Warning),
                "error" => Ok(Severity::Error),
                "fatal" => Ok(Severity::Fatal),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Severity variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl Severity {
    /// 🔢️ Stable numeric mirror of declaration order, 0..3.
    pub fn as_u8(self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
            Severity::Fatal => 3,
        }
    }

    /// 🔢️ Inverse of [`as_u8`](Self::as_u8); `None` for any value outside 0..3.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Severity::Info),
            1 => Some(Severity::Warning),
            2 => Some(Severity::Error),
            3 => Some(Severity::Fatal),
            _ => None,
        }
    }
}

/// 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for ExpectedSet {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_expected(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("tokens".to_string(), DslValue::Array(self.tokens.iter().cloned().map(DslValue::String).collect())),
            ("keywords".to_string(), DslValue::Array(self.keywords.iter().cloned().map(DslValue::String).collect())),
            ("keys".to_string(), DslValue::Array(self.keys.iter().cloned().map(DslValue::String).collect())),
        ])
    }
}
impl FromValue for ExpectedSet {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_expected(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ExpectedSet, found {other:?}"))),
        };
        let strings = |key: &str| -> Result<Vec<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(Vec::new()),
                Some(DslValue::Array(items)) => items
                    .iter()
                    .map(|item| match item {
                        DslValue::String(text) => Ok(text.clone()),
                        other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string in ExpectedSet.{key}, found {other:?}"))),
                    })
                    .collect(),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for ExpectedSet.{key}, found {other:?}"))),
            }
        };
        Ok(ExpectedSet { tokens: strings("tokens")?, keywords: strings("keywords")?, keys: strings("keys")? })
    }
}

/// 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` at API boundaries that predate diagnostics.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: FaultCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    pub expected: Option<ExpectedSet>,
    pub scope: FaultScope,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(self.message, self.span, expected),
            None => TextError::new(self.message, self.span),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Diagnostic {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_diagnostic(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("span".to_string(), self.span.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
        ];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), expected.to_value()));
        }
        entries.push(("scope".to_string(), self.scope.to_value()));
        DslValue::Object(entries)
    }
}
impl FromValue for Diagnostic {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_diagnostic(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Diagnostic, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Diagnostic.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Diagnostic.message, found {other:?}"))),
        };
        Ok(Diagnostic {
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            span: TextSpan::from_value(required("span")?)?,
            message,
            expected: match find("expected") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(ExpectedSet::from_value(slot)?),
            },
            scope: match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            },
        })
    }
}

//#region 🔖️Fault
/// 🧭️ Which layer of the os stack produced a {@link Fault}.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOrigin {
    Edge,
    Renderer,
    Os,
    Module,
    Plugin,
    App,
    Extension,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.3: origin for the
    /// five frozen `surface.*`/`viewer.*` fault codes (`AppRouter`/`OpeningResolver`/`VcsArtifactApp`
    /// role guard) — additive variant, no existing variant touched, no match site in this crate is
    /// exhaustive over it (verified with a repo-wide grep before adding).
    Framework,
}

/// 🎯️ Optional ids locating a fault/diagnostic to a plugin app surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FaultScope {
    pub plugin_id: Option<String>,
    pub app_id: Option<String>,
    pub instance_id: Option<String>,
    pub module: Option<String>,
    pub body_key: Option<String>,
}

/// 🔗️ One hop in a {@link Fault} cause chain.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultCause {
    pub message: String,
    pub code: Option<FaultCode>,
}

/// 🧯️ Structured abort report crossing every os boundary.
/// Scope metadata owns a separate allocation to keep error return values compact.
#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub origin: FaultOrigin,
    pub code: FaultCode,
    pub severity: Severity,
    pub message: String,
    pub scope: Box<FaultScope>,
    pub span: Option<TextSpan>,
    pub causes: Vec<FaultCause>,
    pub retryable: bool,
}

/// 🌉️ Owned Value conversion preserves the canonical Diagnostic fault wire.
/// Literal camelCase keys and absent optional fields match the declared fault schema.
impl ToValue for FaultOrigin {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_origin(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            FaultOrigin::Edge => "edge",
            FaultOrigin::Renderer => "renderer",
            FaultOrigin::Os => "os",
            FaultOrigin::Module => "module",
            FaultOrigin::Plugin => "plugin",
            FaultOrigin::App => "app",
            FaultOrigin::Extension => "extension",
            FaultOrigin::Framework => "framework",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for FaultOrigin {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_origin(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "edge" => Ok(FaultOrigin::Edge),
                "renderer" => Ok(FaultOrigin::Renderer),
                "os" => Ok(FaultOrigin::Os),
                "module" => Ok(FaultOrigin::Module),
                "plugin" => Ok(FaultOrigin::Plugin),
                "app" => Ok(FaultOrigin::App),
                "extension" => Ok(FaultOrigin::Extension),
                "framework" => Ok(FaultOrigin::Framework),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FaultOrigin variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultScope {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_scope(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        for (key, slot) in [
            ("pluginId", &self.plugin_id),
            ("appId", &self.app_id),
            ("instanceId", &self.instance_id),
            ("module", &self.module),
            ("bodyKey", &self.body_key),
        ] {
            if let Some(text) = slot {
                entries.push((key.to_string(), DslValue::String(text.clone())));
            }
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultScope {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_scope(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultScope, found {other:?}"))),
        };
        let take = |key: &str| -> Result<Option<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(None),
                Some(DslValue::String(text)) => Ok(Some(text.clone())),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultScope.{key}, found {other:?}"))),
            }
        };
        Ok(FaultScope {
            plugin_id: take("pluginId")?,
            app_id: take("appId")?,
            instance_id: take("instanceId")?,
            module: take("module")?,
            body_key: take("bodyKey")?,
        })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultCause {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_cause(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone()))];
        if let Some(code) = &self.code {
            entries.push(("code".to_string(), code.to_value()));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultCause {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_cause(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultCause, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCause.message, found {other:?}"))),
        };
        let code = match find("code") {
            None | Some(DslValue::Null) => None,
            Some(slot) => Some(FaultCode::from_value(slot)?),
        };
        Ok(FaultCause { message, code })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Fault {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_fault(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("origin".to_string(), self.origin.to_value()),
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
            ("scope".to_string(), self.scope.to_value()),
        ];
        if let Some(span) = &self.span {
            entries.push(("span".to_string(), span.to_value()));
        }
        if !self.causes.is_empty() {
            entries.push(("causes".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));
        }
        entries.push(("retryable".to_string(), DslValue::Bool(self.retryable)));
        DslValue::Object(entries)
    }
}
impl FromValue for Fault {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_fault(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Fault, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Fault.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Fault.message, found {other:?}"))),
        };
        Ok(Fault {
            origin: FaultOrigin::from_value(required("origin")?)?,
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            message,
            scope: Box::new(match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            }),
            span: match find("span") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(TextSpan::from_value(slot)?),
            },
            causes: match find("causes") {
                None | Some(DslValue::Null) => Vec::new(),
                Some(DslValue::Array(items)) => items.into_iter().map(FaultCause::from_value).collect::<Result<_, _>>()?,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for Fault.causes, found {other:?}"))),
            },
            retryable: match find("retryable") {
                None | Some(DslValue::Null) => false,
                Some(DslValue::Bool(flag)) => flag,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a bool for Fault.retryable, found {other:?}"))),
            },
        })
    }
}

impl From<&str> for Fault {
    fn from(value: &str) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl From<String> for Fault {
    fn from(value: String) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl Fault {
    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {
        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), retryable: false }
    }

    pub fn with_scope(mut self, scope: FaultScope) -> Self {
        *self.scope = scope;
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// 🗣️ Canonical one-line rendering of a fault for `String` error channels.
    ///
    /// `Fault` deliberately has no `Display`: a structured abort must not be silently interpolated
    /// into prose, and `to_string()` would hide the {@link FaultCode} every triage tool keys on.
    /// Every boundary that has to collapse a fault into text calls this instead, so the
    /// `code: message` shape is written once rather than re-derived per crate.
    pub fn describe(&self) -> String {
        format!("{}: {}", self.code.0, self.message)
    }
}

/// 🔁️ Maps a domain error enum into a {@link Fault} at a boundary.
pub trait FaultFrom {
    fn fault_origin(&self) -> FaultOrigin;
    fn fault_code(&self) -> FaultCode;
    fn fault_severity(&self) -> Severity;
    fn fault_message(&self) -> String;
    fn fault_scope(&self) -> FaultScope {
        FaultScope::default()
    }
    fn fault_span(&self) -> Option<TextSpan> {
        None
    }
    fn fault_causes(&self) -> Vec<FaultCause> {
        Vec::new()
    }
    fn fault_retryable(&self) -> bool {
        false
    }

    fn into_fault(self) -> Fault
    where
        Self: Sized,
    {
        Fault { origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), retryable: self.fault_retryable() }
    }
}

impl FaultFrom for TextError {
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Module
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("module.dsl.text")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.clone()
    }

    fn fault_span(&self) -> Option<TextSpan> {
        Some(self.span)
    }
}

/// 📦️ JSON wire encoding for {@link Fault} crossing host/WIT boundaries.
pub fn encode_fault_bytes(fault: &Fault) -> Vec<u8> {
    serde_json::to_vec(&fault.to_value()).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
}

/// 🌐️ Decodes a {@link Fault} from JSON wire bytes; falls back to an os-level message fault.
pub fn decode_fault_bytes(bytes: &[u8]) -> Fault {
    serde_json::from_slice::<DslValue>(bytes).ok().and_then(|value| Fault::from_value(value).ok()).unwrap_or_else(|| Fault::new(FaultOrigin::Os, "os.fault.decode", String::from_utf8_lossy(bytes)))
}

/// 🔁️ Maps an error type into {@link Fault} with a stable dotted code namespace.
#[macro_export]
macro_rules! fault_from_error {
    ($ty:ty, $origin:expr, $prefix:literal) => {
        impl $crate::FaultFrom for $ty {
            fn fault_origin(&self) -> $crate::FaultOrigin {
                $origin
            }

            fn fault_code(&self) -> $crate::FaultCode {
                $crate::FaultCode::new($prefix)
            }

            fn fault_severity(&self) -> $crate::Severity {
                $crate::Severity::Error
            }

            fn fault_message(&self) -> String {
                self.to_string()
            }
        }
    };
}

//#endregion 🔖️Fault
//#endregion 🔖️Errors

//#region 🔖️Limits
/// 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

//#region 🔖️FaultDescribeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️fault-describe/🦀️.rs"]
mod fault_describe_tests;
#[cfg(test)]
#[path = "🧪️tests/🎛️controlled/🦀️.rs"]
mod controlled_tests;
//#endregion 🔖️FaultDescribeTests

#[cfg(test)]
#[path = "🚧️text-error/🧪️tests/🦀️.rs"]
mod text_error_refusal_tests;

```

## corpus-fixture.json

19396 bytes; SHA-256 df8166ecf22ac0d0d5020c0fb342b82112fe0dbc1dcc25117c1e2160da8940e8

```
{
  "cases": [
    {
      "id": "invalidValue new",
      "kind": "invalidValue",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invalidValue",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invalidValue expected",
      "kind": "invalidValue",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invalidValue",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invalidValue fromValueError",
      "kind": "invalidValue",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invalidValue",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invalidValue fromDiagnostic",
      "kind": "invalidValue",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invalidValue",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "canceled new",
      "kind": "canceled",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "canceled",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "canceled expected",
      "kind": "canceled",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "canceled",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "canceled fromValueError",
      "kind": "canceled",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "canceled",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "canceled fromDiagnostic",
      "kind": "canceled",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "canceled",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "ownershipLimit new",
      "kind": "ownershipLimit",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "ownershipLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "ownershipLimit expected",
      "kind": "ownershipLimit",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "ownershipLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "ownershipLimit fromValueError",
      "kind": "ownershipLimit",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "ownershipLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "ownershipLimit fromDiagnostic",
      "kind": "ownershipLimit",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "ownershipLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "allocationFailed new",
      "kind": "allocationFailed",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "allocationFailed",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "allocationFailed expected",
      "kind": "allocationFailed",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "allocationFailed",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "allocationFailed fromValueError",
      "kind": "allocationFailed",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "allocationFailed",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "allocationFailed fromDiagnostic",
      "kind": "allocationFailed",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "allocationFailed",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "workLimit new",
      "kind": "workLimit",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "workLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "workLimit expected",
      "kind": "workLimit",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "workLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "workLimit fromValueError",
      "kind": "workLimit",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "workLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "workLimit fromDiagnostic",
      "kind": "workLimit",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "workLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "depthLimit new",
      "kind": "depthLimit",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "depthLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "depthLimit expected",
      "kind": "depthLimit",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "depthLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "depthLimit fromValueError",
      "kind": "depthLimit",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "depthLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "depthLimit fromDiagnostic",
      "kind": "depthLimit",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "depthLimit",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "unsupportedOwner new",
      "kind": "unsupportedOwner",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "unsupportedOwner",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "unsupportedOwner expected",
      "kind": "unsupportedOwner",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "unsupportedOwner",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "unsupportedOwner fromValueError",
      "kind": "unsupportedOwner",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "unsupportedOwner",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "unsupportedOwner fromDiagnostic",
      "kind": "unsupportedOwner",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "unsupportedOwner",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invariantViolated new",
      "kind": "invariantViolated",
      "operation": "new",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invariantViolated",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invariantViolated expected",
      "kind": "invariantViolated",
      "operation": "expected",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invariantViolated",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "identifier"
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invariantViolated fromValueError",
      "kind": "invariantViolated",
      "operation": "fromValueError",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invariantViolated",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        }
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    },
    {
      "id": "invariantViolated fromDiagnostic",
      "kind": "invariantViolated",
      "operation": "fromDiagnostic",
      "message": "field.3.native decoding canceled",
      "span": {
        "line": 7,
        "column": 3,
        "length": 2
      },
      "expectedWire": {
        "kind": "invariantViolated",
        "message": "field.3.native decoding canceled",
        "span": {
          "line": 7,
          "column": 3,
          "length": 2
        },
        "expected": "name or type= or ="
      },
      "expectedDisplay": "field.3.native decoding canceled at 7:3"
    }
  ],
  "wireCases": [
    {
      "id": "missing mandatory kind",
      "input": {
        "message": "bad",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        }
      },
      "accepted": false
    },
    {
      "id": "unknown kind",
      "input": {
        "kind": "cancelled",
        "message": "bad",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        }
      },
      "accepted": false
    },
    {
      "id": "kind is number",
      "input": {
        "kind": 3,
        "message": "bad",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        }
      },
      "accepted": false
    },
    {
      "id": "negative line",
      "input": {
        "kind": "invalidValue",
        "message": "bad",
        "span": {
          "line": -1,
          "column": 2,
          "length": 3
        }
      },
      "accepted": false
    }
  ],
  "limits": [
    {
      "operation": "bytes",
      "kind": "ownershipLimit",
      "expectedSpan": {
        "line": 1,
        "column": 1,
        "length": 0
      }
    },
    {
      "operation": "depth",
      "kind": "depthLimit",
      "expectedSpan": {
        "line": 7,
        "column": 3,
        "length": 0
      }
    },
    {
      "operation": "tokens",
      "kind": "workLimit",
      "expectedSpan": {
        "line": 7,
        "column": 3,
        "length": 0
      }
    },
    {
      "operation": "nodes",
      "kind": "workLimit",
      "expectedSpan": {
        "line": 7,
        "column": 3,
        "length": 0
      }
    }
  ]
}

```

## package-project.json

1057 bytes; SHA-256 80deda8b6ccf01f368735b705c6e04b39d8f190de229f78cf5bd43a18f232d7d

```
{
  "name": "@semio-tech/framework-diagnostic-rs",
  "$schema": "../../../../../node_modules/nx/schemas/project-schema.json",
  "sourceRoot": "🧰️framework/🔨️modules/⚠️diagnostic",
  "targets": {
    "test-controlled-oracle": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-controlled-oracle"
      }
    },
    "test-native": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-native"
      }
    },
    "test-text-error-portable": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-text-error-portable"
      }
    }
  }
}

```

## diagnostic.ts

138 bytes; SHA-256 b13be9048aaa360619948495e97e90acbf4003ef5ab294bb9430da498ff233a5

```
export { TextError, type TextErrorWire } from "./🚧️text-error/🟦️.ts";
export type { TextSpan } from "./📍️span/🟦️.ts";

```

## controlled-native-tests.rs

9776 bytes; SHA-256 f7c9c871246b24c5460b7e7b456c2b1e924649bc67c471f133817087e0fa280f

```
use super::*;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, Number};
use std::{io::Write, process::{Command, Stdio}};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎛️controlled/🔣️.json")).unwrap()
}

fn refusal(error: semio_framework_value::ValueError, operation: &str) {
    assert_eq!(error.kind.as_str(), fixture()["refusalKinds"][operation].as_str().unwrap());
}

fn intrinsic(value: &serde_json::Value) -> DslValue {
    match value {
        serde_json::Value::Null => DslValue::Null,
        serde_json::Value::Bool(value) => DslValue::Bool(*value),
        serde_json::Value::Number(value) => DslValue::Number(if let Some(value) = value.as_u64() { Number::UInt(value) } else if let Some(value) = value.as_i64() { Number::Int(value) } else { Number::Float(value.as_f64().unwrap()) }),
        serde_json::Value::String(value) => DslValue::String(value.clone()),
        serde_json::Value::Array(values) => DslValue::Array(values.iter().map(intrinsic).collect()),
        serde_json::Value::Object(values) => DslValue::Object(values.iter().map(|(key, value)| (key.clone(), intrinsic(value))).collect()),
    }
}

fn projection(value: &DslValue) -> serde_json::Value {
    match value {
        DslValue::Null => serde_json::Value::Null,
        DslValue::Bool(value) => (*value).into(),
        DslValue::Number(Number::UInt(value)) => (*value).into(),
        DslValue::Number(Number::Int(value)) => (*value).into(),
        DslValue::Number(Number::Float(value)) => (*value).into(),
        DslValue::String(value) => value.clone().into(),
        DslValue::Array(values) => values.iter().map(projection).collect(),
        DslValue::Object(values) => values.iter().map(|(key, value)| (key.clone(), projection(value))).collect::<serde_json::Map<_, _>>().into(),
        DslValue::Bytes(_) => panic!("diagnostic wire has no octets"),
    }
}

fn law<T: FromValue + ToValue + std::fmt::Debug + PartialEq>(row: &serde_json::Value) {
    let input = intrinsic(&row["input"]);
    let mut accept = |_| true;
    let mut control = NativeDecodeControl::new(fixture()["stress"]["maximumBytes"].as_u64().unwrap() as usize, &mut accept);
    let actual = T::from_value_controlled(&input, &mut control);
    if !row["accepted"].as_bool().unwrap() {
        refusal(actual.unwrap_err(), "malformed");
        return;
    }
    let actual = actual.unwrap();
    assert_eq!(projection(&actual.to_value()), row["expected"], "{}", row["id"]);
    let admitted = control.owned_bytes();
    let mut exact = NativeDecodeControl::new(admitted, &mut accept);
    assert_eq!(T::from_value_controlled(&input, &mut exact).unwrap(), actual);
    if admitted > 0 {
        let mut narrow = NativeDecodeControl::new(admitted - 1, &mut accept);
        refusal(T::from_value_controlled(&input, &mut narrow).unwrap_err(), "ownership");
        assert!(narrow.owned_bytes() < admitted);
    }
    let mut accept = |_| true;
    let mut control = NativeEncodeControl::new(fixture()["stress"]["maximumBytes"].as_u64().unwrap() as usize, &mut accept);
    let encoded = actual.to_value_controlled(&mut control).unwrap();
    assert_eq!(projection(&encoded), row["expected"], "{}", row["id"]);
    assert_eq!(encoded, actual.to_value());
    let admitted = control.owned_bytes();
    let mut exact = NativeEncodeControl::new(admitted, &mut accept);
    assert_eq!(actual.to_value_controlled(&mut exact).unwrap(), encoded);
    if admitted > 0 {
        let mut narrow = NativeEncodeControl::new(admitted - 1, &mut accept);
        refusal(actual.to_value_controlled(&mut narrow).unwrap_err(), "ownership");
        assert!(narrow.owned_bytes() < admitted);
    }
    let mut cancel = |_| false;
    refusal(T::from_value_controlled(&input, &mut NativeDecodeControl::new(0, &mut cancel)).unwrap_err(), "cancel");
    let mut cancel = |_| false;
    refusal(actual.to_value_controlled(&mut NativeEncodeControl::new(0, &mut cancel)).unwrap_err(), "cancel");
}

#[test]
fn controlled_diagnostic_all_owner_corpus_and_exact_admission() {
    for row in fixture()["cases"].as_array().unwrap() {
        match row["owner"].as_str().unwrap() {
            "TextSpan" => law::<TextSpan>(row), "TextError" => law::<TextError>(row),
            "FaultCode" => law::<FaultCode>(row), "Severity" => law::<Severity>(row),
            "FaultOrigin" => law::<FaultOrigin>(row), "ExpectedSet" => law::<ExpectedSet>(row),
            "Diagnostic" => law::<Diagnostic>(row), "FaultScope" => law::<FaultScope>(row),
            "FaultCause" => law::<FaultCause>(row), "Fault" => law::<Fault>(row),
            _ => panic!("closed diagnostic owner"),
        }
    }
    println!("[DEBUG] controlled Diagnostic all ten owners preserve invalid, quota and cancellation kinds with exact admission");
}

#[test]
fn controlled_diagnostic_independent_ajv_schema_oracle() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🎛️controlled/🔣️.json")).unwrap();
    let value_schema: serde_json::Value = serde_json::from_str(include_str!("../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json")).unwrap();
    let script = "import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text()),ajv=new Ajv({strict:true}).addSchema(x.valueSchema);if(!ajv.validate(x.schema,x.fixture))throw Error(JSON.stringify(ajv.errors));const rows=x.fixture.cases.map(row=>{const valid=ajv.getSchema(x.schema.$id+'#/$defs/'+row.owner);if(valid(row.input)!==row.accepted)throw Error(row.id);if(row.accepted&&!valid(row.expected))throw Error(row.id+' output');return row.accepted;});await Bun.write(Bun.stdout,JSON.stringify(rows));";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"valueSchema":value_schema,"fixture":fixture()}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Vec<bool> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, fixture()["cases"].as_array().unwrap().iter().map(|row| row["accepted"].as_bool().unwrap()).collect::<Vec<_>>());
}

#[test]
fn controlled_diagnostic_duplicate_key_refusal() {
    let mut accept = |_| true;
    for row in fixture()["duplicateCases"].as_array().unwrap() {
        let value = DslValue::Object(row["entries"].as_array().unwrap().iter().map(|entry| (entry[0].as_str().unwrap().to_string(), intrinsic(&entry[1]))).collect());
        let mut control = NativeDecodeControl::new(4096, &mut accept);
        let rejected = match row["owner"].as_str().unwrap() {
            "FaultCause" => FaultCause::from_value_controlled(&value, &mut control).is_err(),
            "FaultScope" => FaultScope::from_value_controlled(&value, &mut control).is_err(),
            "TextSpan" => TextSpan::from_value_controlled(&value, &mut control).is_err(),
            _ => panic!("closed duplicate owner"),
        };
        assert!(rejected, "{}", row["id"]);
    }
}

#[test]
fn controlled_diagnostic_long_text_lists_and_interior_cancellation() {
    let fixture = fixture(); let stress = &fixture["stress"];
    let maximum = stress["maximumBytes"].as_u64().unwrap() as usize;
    let count = stress["listCount"].as_u64().unwrap() as usize;
    let cutoff = stress["cancelAt"].as_u64().unwrap() as usize;
    let message = stress["text"].as_str().unwrap().repeat(stress["textRepetitions"].as_u64().unwrap() as usize);
    let mut fault = Fault::new(FaultOrigin::Framework, "framework.controlled", message);
    fault.causes = (0..count).map(|index| FaultCause { message: index.to_string(), code: Some(FaultCode::new("cause")) }).collect();
    let value = fault.to_value(); let mut accept = |_| true;
    let decoded = Fault::from_value_controlled(&value, &mut NativeDecodeControl::new(maximum, &mut accept)).unwrap();
    assert_eq!(decoded, fault);
    let mut accept = |_| true;
    assert_eq!(fault.to_value_controlled(&mut NativeEncodeControl::new(maximum, &mut accept)).unwrap(), value);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| { if p.completed >= cutoff && p.total == fault.message.len() { reached = true; false } else { true } };
    refusal(Fault::from_value_controlled(&value, &mut NativeDecodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_encoding::NativeEncodeProgress| { if p.completed >= cutoff && p.total == fault.message.len() { reached = true; false } else { true } };
    refusal(fault.to_value_controlled(&mut NativeEncodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| { if p.completed == 256 && p.total == count { reached = true; false } else { true } };
    refusal(Fault::from_value_controlled(&value, &mut NativeDecodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_encoding::NativeEncodeProgress| { if p.completed == 256 && p.total == count { reached = true; false } else { true } };
    refusal(fault.to_value_controlled(&mut NativeEncodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    println!("[DEBUG] controlled Diagnostic long UTF-8, 1024 causes and interior cancellation retain typed authority");
}

```

## package-script.ts

2222 bytes; SHA-256 7c74e64fdb158f630dee26f50d32c4f51e9905170e77a1ae36daed615a5dda91

```
#!/usr/bin/env bun
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { TestScript } from "../../🧪️tests/🎛️controlled/🟦️.ts";

/** ⚠️ Executes the complete original diagnostic and span laws. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-diagnostic"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

/** 🚧️ Checks the owned typed source error against the neutral schema and independent references. */
class TextErrorPortableScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-text-error-portable");
  const test = resolve(this.root, "../../🚧️text-error/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], this.repoRoot, "diagnostic:text-error:types", 30000);
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", test], this.repoRoot, "diagnostic:text-error:portable", 30000);
 }
}

const router = new ScriptRouter(import.meta.dir).register("test-native", NativeScript).register("test-controlled-oracle", TestScript).register("test-text-error-portable", TextErrorPortableScript);
await runScriptMain(router, { defaultCommand: "test-native" });

```

## controlled-oracle.ts

1942 bytes; SHA-256 0f0fd673fe58532ca2088ef814380870700ef276ef83a7b52fb5e0ec73e70adb

```
import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020.js";
import ts from "typescript";
import { BundleScript } from "../../../🏃️process/🧭️routing/🟦️.ts";
import valueSchema from "../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🎛️controlled/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🎛️controlled/🔣️.json" with { type: "json" };

/** 🎛️ Validates the closed neutral Diagnostic cases with an independent schema engine. */
export class TestScript extends BundleScript {
  async run(): Promise<void> {
    const program = ts.createProgram([import.meta.filename], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, resolveJsonModule: true, allowImportingTsExtensions: true, esModuleInterop: true, skipLibCheck: true, noEmit: true, types: ["bun-types"] });
    assert.deepEqual(ts.getPreEmitDiagnostics(program).map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n")), []);
    const ajv = new Ajv({ strict: true }).addSchema(valueSchema);
    assert(ajv.validate(schema, fixture), JSON.stringify(ajv.errors));
    for (const row of fixture.cases) {
      const validate = ajv.getSchema(`${schema.$id}#/$defs/${row.owner}`);
      assert(validate);
      assert.equal(validate(row.input), row.accepted, row.id);
      if ("expected" in row) assert(validate(row.expected), `${row.id} output`);
    }
    const uniqueKeys = ajv.compile({ type: "array", items: { type: "string" }, uniqueItems: true });
    for (const row of fixture.duplicateCases) assert.equal(uniqueKeys(row.entries.map(entry => entry[0])), false, row.id);
    console.log(`[DEBUG] Diagnostic controlled strict TypeScript and independent Ajv oracle: ${fixture.cases.length} cases`);
  }
}

```

## corpus-schema.json

2572 bytes; SHA-256 f15f0a9dd30c60007851c9498a72fa48920146708817ad375c5ae8772e4de8e0

```
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://semio.tech/schema/diagnostic/text-error-refusal-corpus",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "cases",
    "wireCases",
    "limits"
  ],
  "properties": {
    "cases": {
      "type": "array",
      "minItems": 32,
      "maxItems": 32,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "kind",
          "operation",
          "message",
          "span",
          "expectedWire",
          "expectedDisplay"
        ],
        "properties": {
          "id": {
            "type": "string"
          },
          "kind": {
            "$ref": "https://semio.tech/schema/value/refusal#/$defs/kind"
          },
          "operation": {
            "enum": [
              "new",
              "expected",
              "fromValueError",
              "fromDiagnostic"
            ]
          },
          "message": {
            "type": "string"
          },
          "span": {
            "$ref": "https://semio.tech/schema/diagnostic/controlled#/$defs/TextSpan"
          },
          "expectedWire": {
            "$ref": "https://semio.tech/schema/diagnostic/controlled#/$defs/TextError"
          },
          "expectedDisplay": {
            "type": "string"
          }
        }
      }
    },
    "wireCases": {
      "type": "array",
      "minItems": 4,
      "maxItems": 4,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "input",
          "accepted"
        ],
        "properties": {
          "id": {
            "type": "string"
          },
          "input": {},
          "accepted": {
            "type": "boolean"
          }
        }
      }
    },
    "limits": {
      "type": "array",
      "minItems": 4,
      "maxItems": 4,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "operation",
          "kind",
          "expectedSpan"
        ],
        "properties": {
          "operation": {
            "enum": [
              "bytes",
              "depth",
              "tokens",
              "nodes"
            ]
          },
          "kind": {
            "$ref": "https://semio.tech/schema/value/refusal#/$defs/kind"
          },
          "expectedSpan": {
            "$ref": "https://semio.tech/schema/diagnostic/controlled#/$defs/TextSpan"
          }
        }
      }
    }
  }
}

```

## controlled-production.rs

15455 bytes; SHA-256 ecb722e81a8c9ec06c861ce483adbf16e542cd46b2f9b0a84bbd13f9512c9161

```
//! 🎛️ Closed Diagnostic projection with caller-owned admission and bounded borrowed field scans.
use super::*;
use semio_framework_value::{DecodedValue, NativeDecodeControl, NativeEncodeControl, Number};
type Fields = DecodedValue<Vec<(String, DslValue)>>;

fn fields<'a, const N: usize>(value: &'a DslValue, names: [&str; N], c: &mut NativeDecodeControl<'_>) -> Result<[Option<&'a DslValue>; N], ValueError> {
    c.scoped_stage(|c| {
        c.checkpoint()?;
        let DslValue::Object(entries) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected diagnostic object")) };
        if entries.len() > N { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown or duplicate diagnostic field")) }
        c.begin_stage(entries.len())?;
        let mut output = [None; N];
        for (name, value) in entries {
            let index = names.iter().position(|candidate| *candidate == name).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown diagnostic field"))?;
            if output[index].replace(value).is_some() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "duplicate diagnostic field")) }
            c.step()?;
        }
        Ok(output)
    })
}
fn required<'a>(value: Option<&'a DslValue>, message: &'static str) -> Result<&'a DslValue, ValueError> { value.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) }
fn text(value: &DslValue, message: &'static str, c: &mut NativeDecodeControl<'_>) -> Result<String, ValueError> {
    c.checkpoint()?;
    let DslValue::String(value) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) };
    c.copy_text(value)
}
fn optional_text(value: Option<&DslValue>, message: &'static str, c: &mut NativeDecodeControl<'_>) -> Result<Option<String>, ValueError> {
    match value { None | Some(DslValue::Null) => { c.checkpoint()?; Ok(None) }, Some(value) => text(value, message, c).map(Some) }
}
fn optional<T: FromValue>(value: Option<&DslValue>, c: &mut NativeDecodeControl<'_>) -> Result<Option<T>, ValueError> {
    match value { None | Some(DslValue::Null) => { c.checkpoint()?; Ok(None) }, Some(value) => T::from_value_controlled(value, c).map(Some) }
}
fn strings(value: Option<&DslValue>, message: &'static str, c: &mut NativeDecodeControl<'_>) -> Result<Vec<String>, ValueError> {
    match value { None | Some(DslValue::Null) => { c.checkpoint()?; Ok(Vec::new()) }, Some(DslValue::Array(_)) => Vec::<String>::from_value_controlled(value.unwrap(), c), Some(_) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) }
}
fn scalar(c: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { c.scoped_stage(|c| { c.begin_stage(1)?; c.step() }) }
fn record(c: &mut NativeEncodeControl<'_>, count: usize, build: impl FnOnce(&mut Fields, &mut NativeEncodeControl<'_>) -> Result<(), ValueError>) -> Result<DslValue, ValueError> {
    c.scoped_stage(|c| {
        c.begin_stage(count)?;
        let mut fields = DslValue::object_encoding_controlled(count, c)?;
        build(&mut fields, c)?;
        c.checkpoint()?;
        Ok(DslValue::Object(fields.take()))
    })
}
fn push<T: ToValue + ?Sized>(fields: &mut Fields, name: &str, value: &T, c: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    let value = value.to_value_controlled(c)?;
    DslValue::push_encoding_controlled(fields.get_mut(), name, value, c)?;
    c.step()
}
pub(super) fn retire<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let mut cursor = semio_framework_value::retirement::owned_retirement(value);
    while !cursor.terminal_is_empty() { cursor.close_step(256, 65536).expect("diagnostic retirement respects bounded grant"); }
}

pub(crate) fn encode_span(value: &TextSpan, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 3, |fields, c| { push(fields, "line", &value.line, c)?; push(fields, "column", &value.column, c)?; push(fields, "length", &value.length, c) })
}
pub(crate) fn decode_span(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<TextSpan, ValueError> {
    let [line, column, length] = fields(value, ["line", "column", "length"], c)?;
    let number = |value: Option<&DslValue>, message, c: &mut NativeDecodeControl<'_>| -> Result<u32, ValueError> {
        scalar(c)?;
        match required(value, message)? {
            DslValue::Number(Number::Float(value)) if value.is_finite() && value.fract() == 0.0 && *value >= 0.0 && *value <= u32::MAX as f64 => Ok(*value as u32),
            DslValue::Number(value) => value.as_u64().and_then(|value| u32::try_from(value).ok()).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an exact u32 for TextSpan")),
            _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a number for TextSpan")),
        }
    };
    Ok(TextSpan { line: number(line, "missing TextSpan.line", c)?, column: number(column, "missing TextSpan.column", c)?, length: number(length, "missing TextSpan.length", c)? })
}
pub(super) fn refusal_kind(value: &DslValue) -> Result<ValueRefusalKind, ValueError> {
    match value { DslValue::String(value) => match value.as_str() { "invalidValue" => Ok(ValueRefusalKind::InvalidValue), "canceled" => Ok(ValueRefusalKind::Canceled), "ownershipLimit" => Ok(ValueRefusalKind::OwnershipLimit), "allocationFailed" => Ok(ValueRefusalKind::AllocationFailed), "workLimit" => Ok(ValueRefusalKind::WorkLimit), "depthLimit" => Ok(ValueRefusalKind::DepthLimit), "unsupportedOwner" => Ok(ValueRefusalKind::UnsupportedOwner), "invariantViolated" => Ok(ValueRefusalKind::InvariantViolated), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown TextError.kind")) }, _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected a string for TextError.kind")) }
}
pub(super) fn encode_text_error(value: &TextError, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 3 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "kind", value.kind.as_str(), c)?; push(fields, "message", &value.message, c)?; push(fields, "span", &value.span, c)?; if let Some(value) = &value.expected { push(fields, "expected", value, c)?; } Ok(()) })
}
pub(super) fn decode_text_error(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<TextError, ValueError> {
    let [kind, message, span, expected] = fields(value, ["kind", "message", "span", "expected"], c)?;
    scalar(c)?;
    let kind = refusal_kind(required(kind, "missing TextError.kind")?)?;
    let message = text(required(message, "missing TextError.message")?, "expected a string for TextError.message", c)?;
    let span = TextSpan::from_value_controlled(required(span, "missing TextError.span")?, c)?;
    let expected = optional_text(expected, "expected a string for TextError.expected", c)?;
    Ok(TextError { kind, message, span, expected })
}
pub(super) fn encode_code(value: &FaultCode, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { value.0.to_value_controlled(c) }
pub(super) fn decode_code(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultCode, ValueError> { text(value, "expected a string for FaultCode", c).map(FaultCode) }
pub(super) fn encode_severity(value: &Severity, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { match value { Severity::Info => "info", Severity::Warning => "warning", Severity::Error => "error", Severity::Fatal => "fatal" }.to_value_controlled(c) }
pub(super) fn decode_severity(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<Severity, ValueError> {
    scalar(c)?;
    match value { DslValue::String(value) => match value.as_str() { "info" => Ok(Severity::Info), "warning" => Ok(Severity::Warning), "error" => Ok(Severity::Error), "fatal" => Ok(Severity::Fatal), _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown Severity variant")) }, _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string for Severity")) }
}
pub(super) fn encode_origin(value: &FaultOrigin, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { match value { FaultOrigin::Edge => "edge", FaultOrigin::Renderer => "renderer", FaultOrigin::Os => "os", FaultOrigin::Module => "module", FaultOrigin::Plugin => "plugin", FaultOrigin::App => "app", FaultOrigin::Extension => "extension", FaultOrigin::Framework => "framework" }.to_value_controlled(c) }
pub(super) fn decode_origin(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultOrigin, ValueError> {
    scalar(c)?;
    match value { DslValue::String(value) => match value.as_str() { "edge" => Ok(FaultOrigin::Edge), "renderer" => Ok(FaultOrigin::Renderer), "os" => Ok(FaultOrigin::Os), "module" => Ok(FaultOrigin::Module), "plugin" => Ok(FaultOrigin::Plugin), "app" => Ok(FaultOrigin::App), "extension" => Ok(FaultOrigin::Extension), "framework" => Ok(FaultOrigin::Framework), _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown FaultOrigin variant")) }, _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string for FaultOrigin")) }
}
pub(super) fn encode_expected(value: &ExpectedSet, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { record(c, 3, |fields, c| { push(fields, "tokens", &value.tokens, c)?; push(fields, "keywords", &value.keywords, c)?; push(fields, "keys", &value.keys, c) }) }
pub(super) fn decode_expected(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<ExpectedSet, ValueError> {
    let [tokens, keywords, keys] = fields(value, ["tokens", "keywords", "keys"], c)?;
    let tokens = strings(tokens, "expected an array for ExpectedSet.tokens", c)?.guard_decoded();
    let keywords = strings(keywords, "expected an array for ExpectedSet.keywords", c)?.guard_decoded();
    let keys = strings(keys, "expected an array for ExpectedSet.keys", c)?.guard_decoded();
    Ok(ExpectedSet { tokens: tokens.take(), keywords: keywords.take(), keys: keys.take() })
}
pub(super) fn encode_scope(value: &FaultScope, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    let values = [("pluginId", &value.plugin_id), ("appId", &value.app_id), ("instanceId", &value.instance_id), ("module", &value.module), ("bodyKey", &value.body_key)];
    record(c, values.iter().filter(|(_, value)| value.is_some()).count(), |fields, c| { for (key, value) in values { if let Some(value) = value { push(fields, key, value, c)?; } } Ok(()) })
}
pub(super) fn decode_scope(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultScope, ValueError> {
    let [plugin_id, app_id, instance_id, module, body_key] = fields(value, ["pluginId", "appId", "instanceId", "module", "bodyKey"], c)?;
    Ok(FaultScope { plugin_id: optional_text(plugin_id, "expected a string for FaultScope.pluginId", c)?, app_id: optional_text(app_id, "expected a string for FaultScope.appId", c)?, instance_id: optional_text(instance_id, "expected a string for FaultScope.instanceId", c)?, module: optional_text(module, "expected a string for FaultScope.module", c)?, body_key: optional_text(body_key, "expected a string for FaultScope.bodyKey", c)? })
}
pub(super) fn encode_cause(value: &FaultCause, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { record(c, 1 + usize::from(value.code.is_some()), |fields, c| { push(fields, "message", &value.message, c)?; if let Some(value) = &value.code { push(fields, "code", value, c)?; } Ok(()) }) }
pub(super) fn decode_cause(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultCause, ValueError> {
    let [message, code] = fields(value, ["message", "code"], c)?;
    Ok(FaultCause { message: text(required(message, "missing FaultCause.message")?, "expected a string for FaultCause.message", c)?, code: optional(code, c)? })
}
pub(super) fn encode_diagnostic(value: &Diagnostic, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 5 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "code", &value.code, c)?; push(fields, "severity", &value.severity, c)?; push(fields, "span", &value.span, c)?; push(fields, "message", &value.message, c)?; if let Some(value) = &value.expected { push(fields, "expected", value, c)?; } push(fields, "scope", &value.scope, c) })
}
pub(super) fn decode_diagnostic(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<Diagnostic, ValueError> {
    let [code, severity, span, message, expected, scope] = fields(value, ["code", "severity", "span", "message", "expected", "scope"], c)?;
    let code = FaultCode::from_value_controlled(required(code, "missing Diagnostic.code")?, c)?.guard_decoded();
    let severity = Severity::from_value_controlled(required(severity, "missing Diagnostic.severity")?, c)?;
    let span = TextSpan::from_value_controlled(required(span, "missing Diagnostic.span")?, c)?;
    let message = text(required(message, "missing Diagnostic.message")?, "expected a string for Diagnostic.message", c)?.guard_decoded();
    let expected = optional::<ExpectedSet>(expected, c)?.guard_decoded();
    let scope = optional::<FaultScope>(scope, c)?.unwrap_or_default().guard_decoded();
    Ok(Diagnostic { code: code.take(), severity, span, message: message.take(), expected: expected.take(), scope: scope.take() })
}
pub(super) fn encode_fault(value: &Fault, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 6 + usize::from(value.span.is_some()) + usize::from(!value.causes.is_empty()), |fields, c| { push(fields, "origin", &value.origin, c)?; push(fields, "code", &value.code, c)?; push(fields, "severity", &value.severity, c)?; push(fields, "message", &value.message, c)?; push(fields, "scope", &*value.scope, c)?; if let Some(value) = &value.span { push(fields, "span", value, c)?; } if !value.causes.is_empty() { push(fields, "causes", &value.causes, c)?; } push(fields, "retryable", &value.retryable, c) })
}
pub(super) fn decode_fault(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<Fault, ValueError> {
    let [origin, code, severity, message, scope, span, causes, retryable] = fields(value, ["origin", "code", "severity", "message", "scope", "span", "causes", "retryable"], c)?;
    let origin = FaultOrigin::from_value_controlled(required(origin, "missing Fault.origin")?, c)?;
    let code = FaultCode::from_value_controlled(required(code, "missing Fault.code")?, c)?.guard_decoded();
    let severity = Severity::from_value_controlled(required(severity, "missing Fault.severity")?, c)?;
    let message = text(required(message, "missing Fault.message")?, "expected a string for Fault.message", c)?.guard_decoded();
    c.charge(size_of::<FaultScope>())?;
    let scope = optional::<FaultScope>(scope, c)?.unwrap_or_default().guard_decoded();
    let span = optional::<TextSpan>(span, c)?;
    let causes = optional::<Vec<FaultCause>>(causes, c)?.unwrap_or_default().guard_decoded();
    let retryable = optional::<bool>(retryable, c)?.unwrap_or(false);
    c.checkpoint()?;
    Ok(Fault { origin, code: code.take(), severity, message: message.take(), scope: Box::new(scope.take()), span, causes: causes.take(), retryable })
}

```

## root-production.rs

33182 bytes; SHA-256 2cc025c8c538cbbe862131c70364ef9549c0b174ec17b0d8b1ccdc2f7e878be6

```
//! ⚠️ Text errors, structured diagnostics, fault reporting, and parse limits.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

pub use crate::span::TextSpan;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
#[path = "🎛️controlled/🦀️.rs"]
pub(crate) mod controlled;

//#region 🔖️Errors
/// 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq)]
pub struct TextError {
    pub kind: ValueRefusalKind,
    pub message: String,
    pub span: TextSpan,
    pub expected: Option<String>,
}

impl std::fmt::Display for TextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}:{}", self.message, self.span.line, self.span.column)
    }
}

impl std::error::Error for TextError {}

/// 🌉️ Hand-written, not derived — same DAG reason as `FaultCode`/`Severity` above. No
/// `#[serde(rename_all = …)]` on the original, so field names stay as declared.
impl ToValue for TextError {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_text_error(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("kind".to_string(), DslValue::String(self.kind.as_str().into())), ("message".to_string(), DslValue::String(self.message.clone())), ("span".to_string(), self.span.to_value())];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), DslValue::String(expected.clone())));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for TextError {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_text_error(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TextError, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let kind = match find("kind") { Some(value) => controlled::refusal_kind(&value)?, None => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "missing TextError.kind")) };
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.message, found {other:?}"))),
        };
        let span = match find("span") {
            Some(slot) => TextSpan::from_value(slot)?,
            None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing TextError.span")),
        };
        let expected = match find("expected") {
            None | Some(DslValue::Null) => None,
            Some(DslValue::String(text)) => Some(text),
            Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.expected, found {other:?}"))),
        };
        Ok(TextError { kind, message, span, expected })
    }
}

impl TextError {
    pub fn new(kind: ValueRefusalKind, message: impl Into<String>, span: TextSpan) -> Self {
        Self { kind, message: message.into(), span, expected: None }
    }

    pub fn expected(kind: ValueRefusalKind, message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { kind, message: message.into(), span, expected: Some(expected.into()) }
    }

    /// 🧭️ Retains owned refusal authority at the caller-authored source position.
    pub fn from_value_error(error: ValueError, span: TextSpan) -> Self { Self::new(error.kind, error.message, span) }

    pub fn from_diagnostic(kind: ValueRefusalKind, diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error(kind)
    }
}

/// 🏷️ Stable dotted fault/diagnostic code (e.g. `module.pack.checksum-mismatch`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaultCode(pub String);

impl FaultCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<&'static str> for FaultCode {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

/// 🌉️ Hand-written, not derived: `FaultCode` is `#[serde(transparent)]`, a shape
/// `#[derive(ToValue, FromValue)]` does not support (see its own module docstring).
impl ToValue for FaultCode {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_code(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.clone())
    }
}
impl FromValue for FaultCode {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_code(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(FaultCode(s)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCode, found {other:?}"))),
        }
    }
}

/// 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

impl From<String> for FaultCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DiagnosticCode> for FaultCode {
    fn from(value: DiagnosticCode) -> Self {
        FaultCode::new(value.0)
    }
}

/// 🚦️ Declaration order is the level order: `#[derive(PartialOrd, Ord)]` makes `Info < Warning <
/// Error < Fatal` a structural fact, not a hand-maintained comparator. `as_u8`/`from_u8` (0..3)
/// give a stable wire-compatible numeric mirror for TS/WIT twins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🌉️ Hand-written, not derived: `Severity` is a plain unit-only "string enum" (`#[serde(rename_all
/// = "camelCase")]` with no `tag`, serde's default bare-string representation) —
/// `#[derive(ToValue, FromValue)]`'s enum path only supports internally-tagged representations.
impl ToValue for Severity {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_severity(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for Severity {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_severity(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "info" => Ok(Severity::Info),
                "warning" => Ok(Severity::Warning),
                "error" => Ok(Severity::Error),
                "fatal" => Ok(Severity::Fatal),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Severity variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl Severity {
    /// 🔢️ Stable numeric mirror of declaration order, 0..3.
    pub fn as_u8(self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
            Severity::Fatal => 3,
        }
    }

    /// 🔢️ Inverse of [`as_u8`](Self::as_u8); `None` for any value outside 0..3.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Severity::Info),
            1 => Some(Severity::Warning),
            2 => Some(Severity::Error),
            3 => Some(Severity::Fatal),
            _ => None,
        }
    }
}

/// 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for ExpectedSet {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_expected(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("tokens".to_string(), DslValue::Array(self.tokens.iter().cloned().map(DslValue::String).collect())),
            ("keywords".to_string(), DslValue::Array(self.keywords.iter().cloned().map(DslValue::String).collect())),
            ("keys".to_string(), DslValue::Array(self.keys.iter().cloned().map(DslValue::String).collect())),
        ])
    }
}
impl FromValue for ExpectedSet {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_expected(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ExpectedSet, found {other:?}"))),
        };
        let strings = |key: &str| -> Result<Vec<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(Vec::new()),
                Some(DslValue::Array(items)) => items
                    .iter()
                    .map(|item| match item {
                        DslValue::String(text) => Ok(text.clone()),
                        other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string in ExpectedSet.{key}, found {other:?}"))),
                    })
                    .collect(),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for ExpectedSet.{key}, found {other:?}"))),
            }
        };
        Ok(ExpectedSet { tokens: strings("tokens")?, keywords: strings("keywords")?, keys: strings("keys")? })
    }
}

/// 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` at API boundaries that predate diagnostics.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: FaultCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    pub expected: Option<ExpectedSet>,
    pub scope: FaultScope,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self, kind: ValueRefusalKind) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(kind, self.message, self.span, expected),
            None => TextError::new(kind, self.message, self.span),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Diagnostic {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_diagnostic(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("span".to_string(), self.span.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
        ];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), expected.to_value()));
        }
        entries.push(("scope".to_string(), self.scope.to_value()));
        DslValue::Object(entries)
    }
}
impl FromValue for Diagnostic {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_diagnostic(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Diagnostic, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Diagnostic.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Diagnostic.message, found {other:?}"))),
        };
        Ok(Diagnostic {
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            span: TextSpan::from_value(required("span")?)?,
            message,
            expected: match find("expected") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(ExpectedSet::from_value(slot)?),
            },
            scope: match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            },
        })
    }
}

//#region 🔖️Fault
/// 🧭️ Which layer of the os stack produced a {@link Fault}.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOrigin {
    Edge,
    Renderer,
    Os,
    Module,
    Plugin,
    App,
    Extension,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.3: origin for the
    /// five frozen `surface.*`/`viewer.*` fault codes (`AppRouter`/`OpeningResolver`/`VcsArtifactApp`
    /// role guard) — additive variant, no existing variant touched, no match site in this crate is
    /// exhaustive over it (verified with a repo-wide grep before adding).
    Framework,
}

/// 🎯️ Optional ids locating a fault/diagnostic to a plugin app surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FaultScope {
    pub plugin_id: Option<String>,
    pub app_id: Option<String>,
    pub instance_id: Option<String>,
    pub module: Option<String>,
    pub body_key: Option<String>,
}

/// 🔗️ One hop in a {@link Fault} cause chain.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultCause {
    pub message: String,
    pub code: Option<FaultCode>,
}

/// 🧯️ Structured abort report crossing every os boundary.
/// Scope metadata owns a separate allocation to keep error return values compact.
#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub origin: FaultOrigin,
    pub code: FaultCode,
    pub severity: Severity,
    pub message: String,
    pub scope: Box<FaultScope>,
    pub span: Option<TextSpan>,
    pub causes: Vec<FaultCause>,
    pub retryable: bool,
}

/// 🌉️ Owned Value conversion preserves the canonical Diagnostic fault wire.
/// Literal camelCase keys and absent optional fields match the declared fault schema.
impl ToValue for FaultOrigin {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_origin(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            FaultOrigin::Edge => "edge",
            FaultOrigin::Renderer => "renderer",
            FaultOrigin::Os => "os",
            FaultOrigin::Module => "module",
            FaultOrigin::Plugin => "plugin",
            FaultOrigin::App => "app",
            FaultOrigin::Extension => "extension",
            FaultOrigin::Framework => "framework",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for FaultOrigin {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_origin(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "edge" => Ok(FaultOrigin::Edge),
                "renderer" => Ok(FaultOrigin::Renderer),
                "os" => Ok(FaultOrigin::Os),
                "module" => Ok(FaultOrigin::Module),
                "plugin" => Ok(FaultOrigin::Plugin),
                "app" => Ok(FaultOrigin::App),
                "extension" => Ok(FaultOrigin::Extension),
                "framework" => Ok(FaultOrigin::Framework),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FaultOrigin variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultScope {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_scope(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        for (key, slot) in [
            ("pluginId", &self.plugin_id),
            ("appId", &self.app_id),
            ("instanceId", &self.instance_id),
            ("module", &self.module),
            ("bodyKey", &self.body_key),
        ] {
            if let Some(text) = slot {
                entries.push((key.to_string(), DslValue::String(text.clone())));
            }
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultScope {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_scope(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultScope, found {other:?}"))),
        };
        let take = |key: &str| -> Result<Option<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(None),
                Some(DslValue::String(text)) => Ok(Some(text.clone())),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultScope.{key}, found {other:?}"))),
            }
        };
        Ok(FaultScope {
            plugin_id: take("pluginId")?,
            app_id: take("appId")?,
            instance_id: take("instanceId")?,
            module: take("module")?,
            body_key: take("bodyKey")?,
        })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultCause {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_cause(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone()))];
        if let Some(code) = &self.code {
            entries.push(("code".to_string(), code.to_value()));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultCause {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_cause(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultCause, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCause.message, found {other:?}"))),
        };
        let code = match find("code") {
            None | Some(DslValue::Null) => None,
            Some(slot) => Some(FaultCode::from_value(slot)?),
        };
        Ok(FaultCause { message, code })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Fault {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_fault(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("origin".to_string(), self.origin.to_value()),
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
            ("scope".to_string(), self.scope.to_value()),
        ];
        if let Some(span) = &self.span {
            entries.push(("span".to_string(), span.to_value()));
        }
        if !self.causes.is_empty() {
            entries.push(("causes".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));
        }
        entries.push(("retryable".to_string(), DslValue::Bool(self.retryable)));
        DslValue::Object(entries)
    }
}
impl FromValue for Fault {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_fault(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Fault, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Fault.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Fault.message, found {other:?}"))),
        };
        Ok(Fault {
            origin: FaultOrigin::from_value(required("origin")?)?,
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            message,
            scope: Box::new(match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            }),
            span: match find("span") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(TextSpan::from_value(slot)?),
            },
            causes: match find("causes") {
                None | Some(DslValue::Null) => Vec::new(),
                Some(DslValue::Array(items)) => items.into_iter().map(FaultCause::from_value).collect::<Result<_, _>>()?,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for Fault.causes, found {other:?}"))),
            },
            retryable: match find("retryable") {
                None | Some(DslValue::Null) => false,
                Some(DslValue::Bool(flag)) => flag,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a bool for Fault.retryable, found {other:?}"))),
            },
        })
    }
}

impl From<&str> for Fault {
    fn from(value: &str) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl From<String> for Fault {
    fn from(value: String) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl Fault {
    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {
        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), retryable: false }
    }

    pub fn with_scope(mut self, scope: FaultScope) -> Self {
        *self.scope = scope;
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// 🗣️ Canonical one-line rendering of a fault for `String` error channels.
    ///
    /// `Fault` deliberately has no `Display`: a structured abort must not be silently interpolated
    /// into prose, and `to_string()` would hide the {@link FaultCode} every triage tool keys on.
    /// Every boundary that has to collapse a fault into text calls this instead, so the
    /// `code: message` shape is written once rather than re-derived per crate.
    pub fn describe(&self) -> String {
        format!("{}: {}", self.code.0, self.message)
    }
}

/// 🔁️ Maps a domain error enum into a {@link Fault} at a boundary.
pub trait FaultFrom {
    fn fault_origin(&self) -> FaultOrigin;
    fn fault_code(&self) -> FaultCode;
    fn fault_severity(&self) -> Severity;
    fn fault_message(&self) -> String;
    fn fault_scope(&self) -> FaultScope {
        FaultScope::default()
    }
    fn fault_span(&self) -> Option<TextSpan> {
        None
    }
    fn fault_causes(&self) -> Vec<FaultCause> {
        Vec::new()
    }
    fn fault_retryable(&self) -> bool {
        false
    }

    fn into_fault(self) -> Fault
    where
        Self: Sized,
    {
        Fault { origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), retryable: self.fault_retryable() }
    }
}

impl FaultFrom for TextError {
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Module
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("module.dsl.text")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.clone()
    }

    fn fault_span(&self) -> Option<TextSpan> {
        Some(self.span)
    }
}

/// 📦️ JSON wire encoding for {@link Fault} crossing host/WIT boundaries.
pub fn encode_fault_bytes(fault: &Fault) -> Vec<u8> {
    serde_json::to_vec(&fault.to_value()).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
}

/// 🌐️ Decodes a {@link Fault} from JSON wire bytes; falls back to an os-level message fault.
pub fn decode_fault_bytes(bytes: &[u8]) -> Fault {
    serde_json::from_slice::<DslValue>(bytes).ok().and_then(|value| Fault::from_value(value).ok()).unwrap_or_else(|| Fault::new(FaultOrigin::Os, "os.fault.decode", String::from_utf8_lossy(bytes)))
}

/// 🔁️ Maps an error type into {@link Fault} with a stable dotted code namespace.
#[macro_export]
macro_rules! fault_from_error {
    ($ty:ty, $origin:expr, $prefix:literal) => {
        impl $crate::FaultFrom for $ty {
            fn fault_origin(&self) -> $crate::FaultOrigin {
                $origin
            }

            fn fault_code(&self) -> $crate::FaultCode {
                $crate::FaultCode::new($prefix)
            }

            fn fault_severity(&self) -> $crate::Severity {
                $crate::Severity::Error
            }

            fn fault_message(&self) -> String {
                self.to_string()
            }
        }
    };
}

//#endregion 🔖️Fault
//#endregion 🔖️Errors

//#region 🔖️Limits
/// 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(ValueRefusalKind::OwnershipLimit, format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(ValueRefusalKind::DepthLimit, format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(ValueRefusalKind::WorkLimit, format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(ValueRefusalKind::WorkLimit, format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

//#region 🔖️FaultDescribeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️fault-describe/🦀️.rs"]
mod fault_describe_tests;
#[cfg(test)]
#[path = "🧪️tests/🎛️controlled/🦀️.rs"]
mod controlled_tests;
//#endregion 🔖️FaultDescribeTests

#[cfg(test)]
#[path = "🚧️text-error/🧪️tests/🦀️.rs"]
mod text_error_refusal_tests;

```

## diagnostic-controlled-fixture.json

12056 bytes; SHA-256 64e6cd0adb6153accecb0b822902940996c63ad1cb98a52036a96969d7099828

```
{
  "cases": [
    {
      "id": "TextSpan full",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2,
        "length": 3
      },
      "accepted": true,
      "expected": {
        "line": 1,
        "column": 2,
        "length": 3
      }
    },
    {
      "id": "FaultCode full",
      "owner": "FaultCode",
      "input": "module.test",
      "accepted": true,
      "expected": "module.test"
    },
    {
      "id": "TextError full",
      "owner": "TextError",
      "input": {
        "kind": "invalidValue",
        "message": "bad 雪",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": "name"
      },
      "accepted": true,
      "expected": {
        "kind": "invalidValue",
        "message": "bad 雪",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": "name"
      }
    },
    {
      "id": "ExpectedSet full",
      "owner": "ExpectedSet",
      "input": {
        "tokens": [
          "=",
          "雪"
        ],
        "keywords": [
          "let"
        ],
        "keys": [
          "name"
        ]
      },
      "accepted": true,
      "expected": {
        "tokens": [
          "=",
          "雪"
        ],
        "keywords": [
          "let"
        ],
        "keys": [
          "name"
        ]
      }
    },
    {
      "id": "FaultScope full",
      "owner": "FaultScope",
      "input": {
        "pluginId": "plugin",
        "appId": "app",
        "instanceId": "instance",
        "module": "module",
        "bodyKey": "body"
      },
      "accepted": true,
      "expected": {
        "pluginId": "plugin",
        "appId": "app",
        "instanceId": "instance",
        "module": "module",
        "bodyKey": "body"
      }
    },
    {
      "id": "FaultCause full",
      "owner": "FaultCause",
      "input": {
        "message": "cause",
        "code": "module.cause"
      },
      "accepted": true,
      "expected": {
        "message": "cause",
        "code": "module.cause"
      }
    },
    {
      "id": "Diagnostic full",
      "owner": "Diagnostic",
      "input": {
        "code": "module.test",
        "severity": "warning",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "message",
        "expected": {
          "tokens": [
            "=",
            "雪"
          ],
          "keywords": [
            "let"
          ],
          "keys": [
            "name"
          ]
        },
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        }
      },
      "accepted": true,
      "expected": {
        "code": "module.test",
        "severity": "warning",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "message",
        "expected": {
          "tokens": [
            "=",
            "雪"
          ],
          "keywords": [
            "let"
          ],
          "keys": [
            "name"
          ]
        },
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        }
      }
    },
    {
      "id": "Fault full",
      "owner": "Fault",
      "input": {
        "origin": "framework",
        "code": "framework.test",
        "severity": "fatal",
        "message": "message",
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        },
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "causes": [
          {
            "message": "cause",
            "code": "module.cause"
          },
          {
            "message": "second"
          }
        ],
        "retryable": true
      },
      "accepted": true,
      "expected": {
        "origin": "framework",
        "code": "framework.test",
        "severity": "fatal",
        "message": "message",
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        },
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "causes": [
          {
            "message": "cause",
            "code": "module.cause"
          },
          {
            "message": "second"
          }
        ],
        "retryable": true
      }
    },
    {
      "id": "Severity info",
      "owner": "Severity",
      "input": "info",
      "accepted": true,
      "expected": "info"
    },
    {
      "id": "Severity warning",
      "owner": "Severity",
      "input": "warning",
      "accepted": true,
      "expected": "warning"
    },
    {
      "id": "Severity error",
      "owner": "Severity",
      "input": "error",
      "accepted": true,
      "expected": "error"
    },
    {
      "id": "Severity fatal",
      "owner": "Severity",
      "input": "fatal",
      "accepted": true,
      "expected": "fatal"
    },
    {
      "id": "FaultOrigin edge",
      "owner": "FaultOrigin",
      "input": "edge",
      "accepted": true,
      "expected": "edge"
    },
    {
      "id": "FaultOrigin renderer",
      "owner": "FaultOrigin",
      "input": "renderer",
      "accepted": true,
      "expected": "renderer"
    },
    {
      "id": "FaultOrigin os",
      "owner": "FaultOrigin",
      "input": "os",
      "accepted": true,
      "expected": "os"
    },
    {
      "id": "FaultOrigin module",
      "owner": "FaultOrigin",
      "input": "module",
      "accepted": true,
      "expected": "module"
    },
    {
      "id": "FaultOrigin plugin",
      "owner": "FaultOrigin",
      "input": "plugin",
      "accepted": true,
      "expected": "plugin"
    },
    {
      "id": "FaultOrigin app",
      "owner": "FaultOrigin",
      "input": "app",
      "accepted": true,
      "expected": "app"
    },
    {
      "id": "FaultOrigin extension",
      "owner": "FaultOrigin",
      "input": "extension",
      "accepted": true,
      "expected": "extension"
    },
    {
      "id": "FaultOrigin framework",
      "owner": "FaultOrigin",
      "input": "framework",
      "accepted": true,
      "expected": "framework"
    },
    {
      "id": "Scope null optional",
      "owner": "FaultScope",
      "input": {
        "pluginId": null
      },
      "accepted": true,
      "expected": {}
    },
    {
      "id": "Expected defaults",
      "owner": "ExpectedSet",
      "input": {
        "tokens": null
      },
      "accepted": true,
      "expected": {
        "tokens": [],
        "keywords": [],
        "keys": []
      }
    },
    {
      "id": "TextError null expected",
      "owner": "TextError",
      "input": {
        "kind": "invalidValue",
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": null
      },
      "accepted": true,
      "expected": {
        "kind": "invalidValue",
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        }
      }
    },
    {
      "id": "Cause null code",
      "owner": "FaultCause",
      "input": {
        "message": "",
        "code": null
      },
      "accepted": true,
      "expected": {
        "message": ""
      }
    },
    {
      "id": "Diagnostic defaults",
      "owner": "Diagnostic",
      "input": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "expected": null,
        "scope": null
      },
      "accepted": true,
      "expected": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "scope": {}
      }
    },
    {
      "id": "Fault defaults",
      "owner": "Fault",
      "input": {
        "origin": "os",
        "code": "os.test",
        "severity": "error",
        "message": "",
        "scope": null,
        "span": null,
        "causes": null,
        "retryable": null
      },
      "accepted": true,
      "expected": {
        "origin": "os",
        "code": "os.test",
        "severity": "error",
        "message": "",
        "scope": {},
        "retryable": false
      }
    },
    {
      "id": "Severity unknown",
      "owner": "Severity",
      "input": "warn",
      "accepted": false
    },
    {
      "id": "Origin unknown",
      "owner": "FaultOrigin",
      "input": "host",
      "accepted": false
    },
    {
      "id": "Code wrong kind",
      "owner": "FaultCode",
      "input": 3,
      "accepted": false
    },
    {
      "id": "Span negative",
      "owner": "TextSpan",
      "input": {
        "line": -1,
        "column": 2,
        "length": 3
      },
      "accepted": false
    },
    {
      "id": "Span fraction",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 1.5,
        "length": 3
      },
      "accepted": false
    },
    {
      "id": "Span overflow",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2,
        "length": 4294967296
      },
      "accepted": false
    },
    {
      "id": "Span missing",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2
      },
      "accepted": false
    },
    {
      "id": "Scope unknown",
      "owner": "FaultScope",
      "input": {
        "other": "x"
      },
      "accepted": false
    },
    {
      "id": "Expected wrong item",
      "owner": "ExpectedSet",
      "input": {
        "tokens": [
          true
        ]
      },
      "accepted": false
    },
    {
      "id": "Cause missing message",
      "owner": "FaultCause",
      "input": {},
      "accepted": false
    },
    {
      "id": "Fault missing code",
      "owner": "Fault",
      "input": {
        "origin": "os",
        "severity": "error",
        "message": "x"
      },
      "accepted": false
    },
    {
      "id": "Diagnostic extra",
      "owner": "Diagnostic",
      "input": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "extra": 1
      },
      "accepted": false
    },
    {
      "id": "TextError wrong expected",
      "owner": "TextError",
      "input": {
        "kind": "invalidValue",
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": []
      },
      "accepted": false
    }
  ],
  "stress": {
    "listCount": 1024,
    "textRepetitions": 70000,
    "text": "雪",
    "maximumBytes": 8388608,
    "cancelAt": 65536
  },
  "duplicateCases": [
    {
      "id": "duplicate cause message",
      "owner": "FaultCause",
      "entries": [
        [
          "message",
          "first"
        ],
        [
          "message",
          "second"
        ]
      ]
    },
    {
      "id": "duplicate scope plugin",
      "owner": "FaultScope",
      "entries": [
        [
          "pluginId",
          "first"
        ],
        [
          "pluginId",
          "second"
        ]
      ]
    },
    {
      "id": "duplicate span line",
      "owner": "TextSpan",
      "entries": [
        [
          "line",
          1
        ],
        [
          "line",
          2
        ],
        [
          "length",
          3
        ]
      ]
    }
  ],
  "refusalKinds": {
    "malformed": "invalidValue",
    "ownership": "ownershipLimit",
    "cancel": "canceled"
  }
}

```

## text-error.ts

1742 bytes; SHA-256 24ac6d31b39b6c90d5e7241407a890850bdc563161ebd5b3b0333f5c2a643959

```
import { ValueError, type ValueRefusalKind } from "../../🌱️value/⚠️refusal/🟦️.ts";
import type { TextSpan } from "../📍️span/🟦️.ts";
/** 🧾️ Closed wire projection of a typed source-positioned error. */
export interface TextErrorWire { readonly kind: ValueRefusalKind; readonly message: string; readonly span: TextSpan; readonly expected?: string }
/** 🚧️ An owned error retains refusal authority independently from its display prose. */
export class TextError extends Error {
  readonly span: TextSpan;
  constructor(public readonly kind: ValueRefusalKind, message: string, span: TextSpan, public readonly expected?: string) { super(message); this.name = "TextError"; this.span = { line: span.line, column: span.column, length: span.length }; }
  /** 🧭️ Requires the caller's source position while retaining the original refusal authority. */
  static fromValueError(error: ValueError, span: TextSpan): TextError { return new TextError(error.kind, error.message, span); }
  /** 🧩️ Adds explicit expected syntax without inferring a refusal kind. */
  static expected(kind: ValueRefusalKind, message: string, span: TextSpan, expected: string): TextError { return new TextError(kind, message, span, expected); }
  /** 🧬️ Publishes the mandatory owned kind and exact optional expected syntax. */
  toWire(): TextErrorWire { return { kind: this.kind, message: this.message, span: { line: this.span.line, column: this.span.column, length: this.span.length }, ...(this.expected === undefined ? {} : { expected: this.expected }) }; }
  /** 🔤️ Matches native source-positioned error display. */
  override toString(): string { return `${this.message} at ${this.span.line}:${this.span.column}`; }
}

```
