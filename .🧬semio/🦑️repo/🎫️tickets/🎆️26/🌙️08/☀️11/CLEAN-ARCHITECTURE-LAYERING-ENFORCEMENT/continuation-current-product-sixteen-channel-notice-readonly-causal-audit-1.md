# Current Product16 Channel Notice Causal Diagnosis

Actual original Product16 failed the English older-guest branch at Shell law line98 after the preceding exact localized detail and ARIA tuple assertions. The exact held16/current Shell test, Kernel Rust, Kernel TS, neutral notice catalog and Dev expectation bodies have zero differences. This is not a current drift or channel21/23 literal issue.

The final original assertion examines `expected` itself: `!expected.contains(code) && !expected.contains("speaks app channel")`. The English expected literal includes `this app speaks app channel`. Thus that conjunct is false independently of any runtime output. Both canonical Kernel implementations and the neutral notice catalog intentionally contain the same English phrase. The raw diagnostic producer separately says `the guest speaks app channel {guest}, the host app channel {host}`. Original equality/ARIA assertions already passed before this failing self-check.

A second source-level consumer issue would be reached after the first is corrected: the same corpus declares hostOffset=-1/+1 cases, but the Shell law ignores hostOffset and sets host=current23 and guest=host+guestOffset. Its refused same-guest/previous-or-next-host cases become23/23 and would contradict expect_err. The original protocol unit law correctly derives BOTH offsets from one base. This second finding is inferred from exact source/corpus, not claimed as executed native failure. All seven corpus cases and declared admitted flags remain unchanged.

Proposal in `current-native-channel-notice-inputs-1/proposal.json` is readyfalse and wholly ticket-local: five full pairs, no overlap with held28. English notice says `this app uses app channel`, preserving both named channels, action and every German label. Kernel Rust/TS/catalog, exact Shell expected copy and Dev regex are updated together. Shell fixture input construction reads guestOffset and hostOffset from the common current23 base, while EVERY original assertion line, including the strongest forbidden-fragment check, remains byte-identical. Raw producer code/message/params/schema/pin23 and original whole scope are untouched. No production writes/compiler or future native acceptance. Own/third-party finite controls and independent admission are still required.

```diff
--- 🧰️framework/🔨️modules/🎠️kernel/🦀️.rs
+++ proposed/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs
@@ -2218,7 +2218,7 @@
     ("mutation.target-mismatch", "The change does not fit the target's current state.", "Die Änderung passt nicht zum aktuellen Zustand des Ziels."),
     ("mutation.too-large", "This change is too large to record at once — split it into smaller steps.", "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen."),
     ("plugin.media.schema-mismatch", "This input is a {found} document, but only {expected} documents can be loaded here.", "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden."),
-    ("plugin.channel-mismatch", "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.", "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."),
+    ("plugin.channel-mismatch", "This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.", "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."),
     ("history-filter.unknown", "This history filter is not known — choose one the history panel offers.", "Dieser Verlaufsfilter ist unbekannt — einen im Verlaufsbereich angebotenen wählen."),
     ("window-transient.window-required", "This needs an open window — focus a window first.", "Dafür wird ein offenes Fenster benötigt — zuerst ein Fenster fokussieren."),
     ("window-transient.window-stale", "The window is no longer open.", "Das Fenster ist nicht mehr geöffnet."),
```
```diff
--- 🧰️framework/🔨️modules/🎠️kernel/🟦️.ts
+++ proposed/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts
@@ -2016,7 +2016,7 @@
   { code: "mutation.target-mismatch", en: "The change does not fit the target's current state.", de: "Die Änderung passt nicht zum aktuellen Zustand des Ziels." },
   { code: "mutation.too-large", en: "This change is too large to record at once — split it into smaller steps.", de: "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." },
   { code: "plugin.media.schema-mismatch", en: "This input is a {found} document, but only {expected} documents can be loaded here.", de: "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden." },
-  { code: "plugin.channel-mismatch", en: "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.", de: "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen." },
+  { code: "plugin.channel-mismatch", en: "This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.", de: "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen." },
   { code: "history-filter.unknown", en: "This history filter is not known — choose one the history panel offers.", de: "Dieser Verlaufsfilter ist unbekannt — einen im Verlaufsbereich angebotenen wählen." },
   { code: "window-transient.window-required", en: "This needs an open window — focus a window first.", de: "Dafür wird ein offenes Fenster benötigt — zuerst ein Fenster fokussieren." },
   { code: "window-transient.window-stale", en: "The window is no longer open.", de: "Das Fenster ist nicht mehr geöffnet." },
```
```diff
--- 🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️framework-notices/🔣️.json
+++ proposed/🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️framework-notices/🔣️.json
@@ -78,7 +78,7 @@
     },
     {
       "code": "plugin.channel-mismatch",
-      "en": "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.",
+      "en": "This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.",
       "de": "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."
     },
     {
```
```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs
+++ proposed/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs
@@ -78,15 +78,16 @@
 fn a_refused_guest_channel_is_told_as_its_localized_notice() {
     let corpus: Value = serde_json::from_str(include_str!("../../../../../../📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake/🔣️.json")).expect("the channel-handshake corpus parses");
     let code = corpus["code"].as_str().expect("code");
-    let refused: Vec<(String, i64)> = corpus["cases"].as_array().expect("cases").iter().filter(|case| case["admitted"] == false).map(|case| (case["name"].as_str().expect("name").to_string(), case["guestOffset"].as_i64().expect("offset"))).collect();
+    let refused: Vec<(String, i64, i64)> = corpus["cases"].as_array().expect("cases").iter().filter(|case| case["admitted"] == false).map(|case| (case["name"].as_str().expect("name").to_string(), case["guestOffset"].as_i64().expect("offset"), case["hostOffset"].as_i64().unwrap_or(0))).collect();
     assert!(!refused.is_empty(), "the corpus names refused guests");
-    for (name, offset) in refused {
-        let host = protocol::CHANNEL_VERSION;
-        let guest = u32::try_from(i64::from(host) + offset).expect("a guest version");
+    for (name, guest_offset, host_offset) in refused {
+        let base = protocol::CHANNEL_VERSION;
+        let host = u32::try_from(i64::from(base) + host_offset).expect("a host version");
+        let guest = u32::try_from(i64::from(base) + guest_offset).expect("a guest version");
         let fault = protocol::admit_guest_channel_version(guest, host).expect_err("the corpus case is refused");
         assert_eq!(fault.code.0, code, "{name}");
         for (locale, locale_id, expected) in [
-            (semio_framework_ui_locale::Locale::En, "en", format!("This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.")),
+            (semio_framework_ui_locale::Locale::En, "en", format!("This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.")),
             (semio_framework_ui_locale::Locale::De, "de", format!("Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen.")),
         ] {
             let mut shell = ShellState::new(Vec::new(), String::new(), locale, semio_framework_ui_locale::Terminology::Native);
```
```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts
+++ proposed/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts
@@ -258,7 +258,7 @@
     noticeLoading: "The document is still loading — wait for it or cancel it first.",
     historyFull: /^This document's history is full \((\d+) edits\)\.$/,
     replayFaulted: "Replay failed: later mutations could not be checked",
-    channelMismatch: /^This plugin was built for app channel \d+, but this app speaks app channel \d+ — rebuild the plugin\.$/,
+    channelMismatch: /^This plugin was built for app channel \d+, but this app uses app channel \d+ — rebuild the plugin\.$/,
     pending: "Not applied while editing",
     targetMissing: "Error: Target missing",
     severityError: /\b(Error|Fatal)\b/,
```
