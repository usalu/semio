# Layout Drawing Composition

The existing Layout background slot had no production EmbeddedChildCodecV1 binding anywhere in the inspected plugin ownership tree. Its SVG/DXF/DWG import/export and composed SQLite snapshot routes therefore refused even though the adapter leaves existed. The formerly referenced layout-drawing composition test folder is absent.

A Layout-owned composition module now binds the existing neutral interface to the canonical stdio Drawing owner. It uses that owner's real decode_drawing/encode_drawing functions and native SQLite component projection/reconstruction, retains Drawing validation and explicit retirement, and keeps both concrete owners out of the Layout schema. Artifact declaration binds before registering its codecs. No duplicate external format parser is introduced.

Native tests were authored first for idempotent binding, unknown format refusal, exact neutral scene lift/lower, and actual official W3C SVG-to-Layout import with one canvas page and retained editable drawing texts. The neutral fixture dimensions come from the independent jsdom XML oracle stored with unmodified accepted upstream inputs. These native tests are not yet executed because the repo frontend remains unavailable.

The current shared EmbeddedChildCodecV1 lower/lift/import/export signatures are synchronous; this binding follows the existing first-party interface. Resumable editor jobs and further bounded codec controls remain part of the active goal. No full rendering or format conformance claim is made by this binding.
