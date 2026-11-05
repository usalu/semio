# Authored Picker Action Availability

Read-only source audit; no UI/browser execution.

The actual custom Select element suppresses same-value selection: its selectValue callback invokes onValueChange only when nextValue differs from value. NavbarExampleSelect delegates directly to this Select. Shell command-bar scope dispatches buildActiveExampleAction only inside that onValueChange callback. Consequently clicking Note while Note is already selected does not dispatch a reload, even after the document was independently edited. A harness requiring that click alone to restore the probe is brittle.

Text has a genuine declared setActiveExample action: its editor builder calls action_with(set_active_example_action), adds exampleId select arguments with the note default, and registers the retained handler/factory. The shared action definition is ActionKind::Mutation in file, without in_palette(false). Its HostOnly publication contract does not itself hide the action. The existing rendered action path with explicit/default note arguments is therefore the appropriate fallback to verify, rather than inventing a direct plugin invocation. Exact component owners: framework/ui/elements/Select, NavbarExampleSelect; renderer/ShellHost/command-bar/scope; stdio/contract/window-kit action helpers; SemioText editor declaration/handler.

NavbarExampleSelect also offers a No example sentinel by default, but this is not certified as a successful fallback: Text's strict native parser refuses empty example IDs, and this audit did not establish a framework blank-selection intercept. An earlier suggestion to use sentinel then Note was qualified immediately to root/runtime. Do not count that route as proven until its actual runtime handler is traced.

No missing native Text setActiveExample handler was found. Actual rendered action availability and successful explicit reload still require the real browser journey.


## No-example interception resolved

A framework blank-document handler does exist: EditorApp::catalogue_example_document checks empty ID and returns reference_birth printed through the actual snapshot codec. However the actual command route only invokes that fallback when setActiveExample is absent from the tool registry (instance/command-routes/staging/actions line196). Text explicitly declares and registers the action/factory, so it bypasses that fallback and reaches its strict native example parser, which rejects the empty ID.

The navbar includes No example by default, normalizes its sentinel to empty string, and shellscope dispatches that empty argument. Therefore for the declared Text tool this is a real invalid UI offer, not a proven working blank intercept. Root/runtime received the exact branch condition and source paths. No product change was proposed or made speculatively. The acceptance harness avoids the sentinel and explicitly loads note through the real action, so this gap is independent of that prepared journey. Actual browser journeys remain0.


## Scope and capability route inventory

All19Semio editor sources call the strict shared example parser and declare the same real example action. Seventeen use explicit example factories; Mesh/Brep route the action through their shared bounded native edit work. The invalid blank offer is therefore not confined to Text. A bounded all-stdio search finds exactly19editor calls to this strict helper, all in Semio; other canonical formats use different handlers, so their blank rejection cannot be inferred from this helper census.

Shell scope does not currently pass includeNoExample, leaving the navbar global default true. Existing declared exampleId select arguments already carry named choices; Semio's choices contain no blank. The smallest metadata-grounded UI route is to honor explicit blank permission in a declared selector's choices, keeping the framework-owned undeclared-selector policy separate. This avoids weakening native empty-ID rejection or adding speculative app capabilities.

Broadening the native blank fallback is not a funding-neutral shortcut: catalogue_example_document calls reference_birth and print, then dispatch_catalogue_example synchronously parses a snapshot, encode_pack and builds load effects before dispatch_emit. Those whole-body operations do not gain bounded original funding merely by moving the branch above the registered handler. Runtime received this caution while owning the actual fix. No production edit or browser/native run was made.
