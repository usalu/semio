# Canonical Value Refusal API Full Before Capture

The actual Rust and TypeScript owner paths remain absent. The schema-first proposed sources have mandatory kind construction, preserve identity during dotted path decoration and permit an explicit terminal text projection only. Neither source contains a compatibility constructor or implicit String conversion. The full authored sources, bytes, SHA-256 and before-absence inverses are preserved below before any mount.

```json
{
  "capturedAt": "2026-10-02T23:02:15.587Z",
  "status": "authored before canonical API source mount; both actual paths remain absent through test-only RED",
  "files": [
    {
      "path": "🧰️framework/🔨️modules/🌱️value/⚠️refusal/🦀️.rs",
      "before": {
        "exists": false
      },
      "authoredSource": "//! ⚠️ Owned refusal identity survives path decoration and controlled codec boundaries.\n/// 🏷️ Stable semantic failure identity declared by the refusal schema.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum ValueRefusalKind { InvalidValue, Canceled, OwnershipLimit, AllocationFailed, WorkLimit, DepthLimit, UnsupportedOwner, InvariantViolated }\nimpl ValueRefusalKind {\n    /// 🔤️ Returns the schema spelling without allocating or inspecting error prose.\n    pub const fn as_str(self) -> &'static str {\n        match self { Self::InvalidValue => \"invalidValue\", Self::Canceled => \"canceled\", Self::OwnershipLimit => \"ownershipLimit\", Self::AllocationFailed => \"allocationFailed\", Self::WorkLimit => \"workLimit\", Self::DepthLimit => \"depthLimit\", Self::UnsupportedOwner => \"unsupportedOwner\", Self::InvariantViolated => \"invariantViolated\" }\n    }\n}\n/// 🚨️ An owned typed refusal with its complete dotted display path.\n#[derive(Clone, Debug, PartialEq, Eq)]\npub struct ValueError { pub kind: ValueRefusalKind, pub message: String }\nimpl ValueError {\n    /// 🧭️ Constructs one refusal with an explicit authority at its actual failure site.\n    pub fn new(kind: ValueRefusalKind, message: impl Into<String>) -> Self { Self { kind, message: message.into() } }\n    /// 🪆️ Adds a parent field or ordinal while retaining the original refusal authority.\n    pub fn under(self, segment: impl std::fmt::Display) -> Self { Self::new(self.kind, format!(\"{segment}.{}\", self.message)) }\n    /// 🗣️ Moves the prose at a declared terminal text-only boundary.\n    pub fn into_message(self) -> String { self.message }\n}\nimpl std::fmt::Display for ValueError { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.message) } }\nimpl std::error::Error for ValueError {}\n",
      "bytes": 1840,
      "sha256": "0fa94c5ea51bfac3c10c5e7c86fbe73781eee05ff65c90b71b1d045fb3d3e4a5",
      "inverse": {
        "operation": "remove exactly this newly mounted owned API source"
      }
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts",
      "before": {
        "exists": false
      },
      "authoredSource": "/** ⚠️ First-party refusal identities declared by the language-neutral schema. */\nexport type ValueRefusalKind = \"invalidValue\" | \"canceled\" | \"ownershipLimit\" | \"allocationFailed\" | \"workLimit\" | \"depthLimit\" | \"unsupportedOwner\" | \"invariantViolated\";\n/** 🚨️ An owned typed refusal retains authority through every parent field or ordinal. */\nexport class ValueError extends Error {\n  constructor(public readonly kind: ValueRefusalKind, message: string) { super(message); this.name = \"ValueError\"; }\n  /** 🪆️ Decorates the dotted display path without changing the semantic identity. */\n  under(segment: string | number | bigint): ValueError { return new ValueError(this.kind, `${segment}.${this.message}`); }\n  /** 🗣️ Projects prose only at a declared terminal text boundary. */\n  intoMessage(): string { return this.message; }\n  /** 🔤️ Renders the same owned message as the native implementation. */\n  override toString(): string { return this.message; }\n}\n",
      "bytes": 983,
      "sha256": "3d4dcfb89874cadecfac7e0a4fe72385eb0de0fd9696fc4ffd2391a91a530031",
      "inverse": {
        "operation": "remove exactly this newly mounted owned API source"
      }
    }
  ]
}
```
