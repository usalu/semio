# Scout Report 1: App-Channel Codec State

## 1. AppCommand Variants & Tags

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` (lines 52-226)

**Declared tags in order (0-29)**:
0=Hello, 1=ConfigCommand, 2=Command, 3=CommandText, 4=RefreshUi, 5=ContextMenu, 6=ArtifactCommand, 7=ApplyEnvelopes, 8=LoadDocument, 9=ReadDocument, 10=LoadConfig, 11=ReadConfig, 12=AttachBackbone, 13=DetachBackbone, 14=MediaIn, 15=MediaOut, 16=MediaFingerprint, 17=Bye, 18=PureCommand, 19=LoadChildren, 20=ReadChildren, 21=ReadHistory, 22=TransactionPrepare, 23=TransactionCommit, 24=TransactionRollback, 25=TransactionUndo, 26=TransactionRedo, 27=OpenArtifact, 28=SetDefaultApp, 29=ClearDefaultApp

**Next free tag**: **30**

## 2. AppFrame Variants & Tags

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` (lines 229-307)

**Declared tags in order (0-22)**:
0=Welcome, 1=Done, 2=Invocation, 3=UiSection, 4=Effects, 5=Events, 6=DocumentChanged, 7=Document, 8=Config, 9=ConfigChanged, 10=ContextMenu, 11=Media, 12=MediaFingerprint, 13=Error, 14=Emit, 15=Draft, 16=Children, 17=Ephemeral, 18=HistorySnapshot, 19=TransactionProposal, 20=TransactionPrepared, 21=TransactionCommitted, 22=TransactionRolledBack

**Next free tag**: **23**

## 3. CHANNEL_VERSION

- **Rust definition**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` line 20: `pub const CHANNEL_VERSION: u32 = 10;`
- **Cross-language pin**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/channel-version.json` line 3: `"channelVersion": 10`
- **TS constant**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts` line 2347: `const APP_CHANNEL_VERSION = 10;`
- **TS assertion test**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts` line 2814 (vitest: "pins APP_CHANNEL_VERSION against the shared cross-language channel version")

## 4. TypeScript Region Line Ranges

- **AppChannelCodec region**: lines 1616–2319 (TS component.ts)
- **AppChannelClient region**: lines 2321–2584 (TS component.ts)

## 5. Variant Drift Check

**TS matches Rust one-for-one**:
- All AppCommand tags 0–29 match (TS: lines 1777–1783, Rust: lines 405–576)
- All AppFrame tags 0–22 match (TS: lines 1784–1788, Rust: lines 699–837)
- **No drift found**

## 6. Golden Fixture Vectors

**Location**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/`

**Files**:
- `app-command-transaction.json` (hex-encoded): 6 vectors (TransactionPrepareOwner, TransactionPreparePrePlanned, TransactionCommit, TransactionRollback, TransactionUndo, TransactionRedo)
- `app-command-opening.json` (hex-encoded): 4 vectors (OpenArtifactResolve, OpenArtifactExplicit, SetDefaultApp, ClearDefaultApp)
- `app-frame-transaction.json` (hex-encoded): 4 vectors (TransactionProposal, TransactionPrepared, TransactionCommitted, TransactionRolledBack)
- `channel-version.json` (JSON): single version number

**Rust test**: `channel_opening_fixtures_match_shared_cross_language_json_vectors` (line 1599)

**TS vitest tests**: 
- "matches the shared cross-language transaction fixture vectors, byte-for-byte" (line 3040)
- "matches the shared cross-language opening fixture vectors, byte-for-byte" (line 3083)

## 7. Invocation & Error Frame Shapes

**Invocation (tag 2)** — Rust line 235:
```rust
{ in_reply_to: u64, output: Vec<u8>, diagnostics: Vec<u8>, ui_scope: Vec<u8>, history_patch: Vec<u8> }
```

**Error (tag 13)** — Rust line 246:
```rust
{ in_reply_to: Option<u64>, fault: Vec<u8> }
```

**Safety for trailing field additions**: Both frames use only primitive/bytes fields in forward order. **Safe to append**: `messages: Vec<u8>` to Invocation, `report: Vec<u8>` to Error. No Option<T> boxing or complex nesting that would impede decoding of new trailing fields.

## 8. ApplyOutcome::Rejected

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs` line 76

```rust
Rejected { reason: String }
```

---

**Status**: All mappings complete. No cross-language drift. Version pin guarding against future desync. Ready for lane 1-C tag appending.
