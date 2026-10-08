# Current Channel Notice Five Pair Native Owner Review

Read-only exact current full beforeimages, SHA256 before/after, sole full forward/inverse joins checked for all five proposed pairs. All detected standalone original assertion lines are conserved. The canonical English message and corresponding expected copies are changed together; original forbidden-fragment assertion/raw producer diagnostic/German corpus are retained. Shell constructor now derives guest and host offsets from the same current channel. This review is source-only; later host-offset behavior has not yet executed natively. Independent actual finite controls and current receipt admission remain pending. No production source writes/compiler/native acceptance.

```diff
--- 

+++ 

@@ -2219,5 +2219,5 @@

     ("mutation.too-large", "This change is too large to record at once — split it into smaller steps.", "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen."),
     ("plugin.media.schema-mismatch", "This input is a {found} document, but only {expected} documents can be loaded here.", "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden."),
-    ("plugin.channel-mismatch", "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.", "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."),
+    ("plugin.channel-mismatch", "This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.", "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."),
     ("history-filter.unknown", "This history filter is not known — choose one the history panel offers.", "Dieser Verlaufsfilter ist unbekannt — einen im Verlaufsbereich angebotenen wählen."),
     ("window-transient.window-required", "This needs an open window — focus a window first.", "Dafür wird ein offenes Fenster benötigt — zuerst ein Fenster fokussieren."),
```

```diff
--- 

+++ 

@@ -2017,5 +2017,5 @@

   { code: "mutation.too-large", en: "This change is too large to record at once — split it into smaller steps.", de: "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." },
   { code: "plugin.media.schema-mismatch", en: "This input is a {found} document, but only {expected} documents can be loaded here.", de: "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden." },
-  { code: "plugin.channel-mismatch", en: "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.", de: "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen." },
+  { code: "plugin.channel-mismatch", en: "This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.", de: "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen." },
   { code: "history-filter.unknown", en: "This history filter is not known — choose one the history panel offers.", de: "Dieser Verlaufsfilter ist unbekannt — einen im Verlaufsbereich angebotenen wählen." },
   { code: "window-transient.window-required", en: "This needs an open window — focus a window first.", de: "Dafür wird ein offenes Fenster benötigt — zuerst ein Fenster fokussieren." },
```

```diff
--- 

+++ 

@@ -79,5 +79,5 @@

     {
       "code": "plugin.channel-mismatch",
-      "en": "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.",
+      "en": "This plugin was built for app channel {guest}, but this app uses app channel {host} — rebuild the plugin.",
       "de": "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."
     },
```

```diff
--- 

+++ 

@@ -79,13 +79,14 @@

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
```

```diff
--- 

+++ 

@@ -259,5 +259,5 @@

     historyFull: /^This document's history is full \((\d+) edits\)\.$/,
     replayFaulted: "Replay failed: later mutations could not be checked",
-    channelMismatch: /^This plugin was built for app channel \d+, but this app speaks app channel \d+ — rebuild the plugin\.$/,
+    channelMismatch: /^This plugin was built for app channel \d+, but this app uses app channel \d+ — rebuild the plugin\.$/,
     pending: "Not applied while editing",
     targetMissing: "Error: Target missing",
```
