// usage: node hooks-spec.cjs <tag> <out.json> : specs of the React hook splits (typed from their producers)
const { R, f, ph, write, dir, doc } = require("./lib_ps.cjs");
const S = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/";
const sh = (name, text) => ({ shared: { dir: dir("🧰", name), doc: doc("🧰", text) } });
const mk = (rel, fn, files, extra = {}) => ({ file: S + rel, function: fn, keep: [], resolve: true, typesFromProducers: true, files, ...extra });
const specs = {
  focus: mk("🏛️ShellHost/🎮️focus-and-input/🟦️.tsx", "useFocusAndInput", [
    f("🎯", "focused-program-and-introduction", "The focused program, history, time travel, introduction and tutorial state of the focus and input hook.", [
      ph("useFocusedProgramAndIntroductionState", "🧭", "Binds the focused program, its history and time travel indicator and the introduction and tutorial state with their effects.", 93, 328),
    ]),
    f("⌨", "registers-and-context-menu", "The utility and tool registers, the introduction callbacks, the browser actor view state and the context menu of the focus and input hook.", [
      ph("useUtilityAndToolRegisters", "🧰", "Builds the utility and tool register cells, the local interaction publishers and the introduction callbacks.", 347, 551),
      ph("useBrowserActorViewAndContextMenu", "🪟", "Posts the browser actor view state, runs the focus and input effects and builds the context menu request.", 567, 746),
    ]),
  ]),
  identity: mk("🏛️ShellHost/🔒️identity-document-ports/🟦️.ts", "useIdentityAndDocumentPorts", [
    f("🪪", "identity-and-directory-state", "The identity, agent bridge and directory state of the identity and document ports hook.", [
      ph("useIdentityAgentBridgeAndDirectoryState", "🆔", "Binds the document access, the agent bridge, the identity, session authority and directory refs and state.", 181, 464),
    ]),
    f("📎", "hub-port-and-document-backbone", "The hub connection port, the inference port and the document backbone ports of the identity and document ports hook.", [
      ph("useHubPortInferenceAndDocumentBackbone", "🔌", "Builds the hub connection port, the inference and presence refs and the document backbone ports.", 465, 661),
      ph("useEffectOwnersAndDialogs", "💬", "Builds the effect owner capture, the document archive ports and the owned dialogs.", 666, 758),
    ]),
  ], { ...sh("identity-port-types", "The shared identity and document port types: the open document session, background index session and retained browser actor records."), allowExportedMoves: true }),
  theme: mk("🏛️ShellHost/🎨️theme-driver-settings/🟦️.tsx", "useThemeDriverSettings", [
    f("🎨", "theme-and-driver-state", "The theme and driver state and the theme save callbacks of the theme driver settings hook.", [
      ph("useThemeAndDriverState", "🌈", "Binds the theme and driver state and the theme selection callbacks.", 101, 285),
      ph("useThemeSaveAndSettingsHostRef", "💾", "Builds the theme save callbacks and the settings host ref.", 290, 402),
    ]),
    f("🛍", "settings-and-marketplace-hosts", "The settings host, the marketplace host and the task manager tab of the theme driver settings hook.", [
      ph("useSettingsHost", "⚙", "Builds the settings host api and the marketplace host ref.", 403, 561),
      ph("useMarketplaceHostAndTaskManager", "🏪", "Builds the marketplace host api and the framework task manager tab.", 562, 711),
    ]),
  ]),
  hub: mk("🔗️HubConnection/🟦️.tsx", "useHubConnection", [
    f("📒", "connection-book-and-commands", "The hub connection book state and the hub command callbacks of the hub connection hook.", [
      ph("useHubConnectionBook", "📖", "Binds the hub connection book state, its readers and the space member watchers.", 36, 200),
      ph("useHubCommandCallbacks", "⌨", "Builds the hub command runner and the sign in cancellation callbacks.", 207, 378),
    ]),
    f("🔑", "sign-in-agent-and-delegation", "The hub sign out, agent client configuration and delegation callbacks of the hub connection hook.", [
      ph("useHubSignInAndAgentCallbacks", "🚪", "Builds the sign out, agent client configuration and clipboard callbacks.", 384, 563),
      ph("useHubDelegationCallbacks", "🤝", "Builds the delegation revocation callback and the delegation rows.", 571, 603),
    ]),
  ], sh("hub-connection-helpers", "The shared hub connection helpers: the live ref hook the connection callbacks read.")),
};
write(specs[process.argv[2]], process.argv[3]);
