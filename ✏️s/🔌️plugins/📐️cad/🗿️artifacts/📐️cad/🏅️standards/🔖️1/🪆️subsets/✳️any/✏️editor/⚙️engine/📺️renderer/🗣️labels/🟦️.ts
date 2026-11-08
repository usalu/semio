/** 🗣️ English and German wording of the CAD REPL panes, registered with the UI translation port (no default language). */

import { registerUiTranslationBundles, useLabel, useLabelFormatter } from "@semio-tech/ui-react";

const row = (normal: string) => ({ label: { normal, beginner: normal } });

const cadReplLabel = registerUiTranslationBundles({
  en: {
    translation: {
      cad: {
        repl: {
          clear: row("Clear"), clearField: row("Clear {{field}}"), attributes: row("Attributes"), enabled: row("Enabled"), stats: row("Stats"), model: row("Model"), properties: row("Properties"), selected: row("{{count}} selected"), selection: row("Selection"),
          editor: row("Editor"), modelDefinition: row("Model definition"), transferFrom: row("Transfer from"), transferTo: row("Transfer to"), showPrimitives: row("Show primitives"), filterPrimitives: row("Filter primitives"),
          showTypologies: row("Show typologies"), selectionTypologies: row("Selection typologies"), lastResponse: row("Last response"), selectIncoming: row("Select incoming transformation…"), selectOutgoing: row("Select outgoing transformation…"),
        },
      },
    },
  },
  de: {
    translation: {
      cad: {
        repl: {
          clear: row("Leeren"), clearField: row("{{field}} leeren"), attributes: row("Attribute"), enabled: row("Aktiviert"), stats: row("Kennzahlen"), model: row("Modell"), properties: row("Eigenschaften"), selected: row("{{count}} ausgewählt"), selection: row("Auswahl"),
          editor: row("Editor"), modelDefinition: row("Modelldefinition"), transferFrom: row("Übertragen von"), transferTo: row("Übertragen nach"), showPrimitives: row("Primitive anzeigen"), filterPrimitives: row("Primitive filtern"),
          showTypologies: row("Typologien anzeigen"), selectionTypologies: row("Auswahltypologien"), lastResponse: row("Letzte Antwort"), selectIncoming: row("Eingehende Transformation wählen…"), selectOutgoing: row("Ausgehende Transformation wählen…"),
        },
      },
    },
  },
});

/** 🏷️ Every REPL label in the viewer's language; call once at the top of a component. */
export function useCadReplLabels() {
  const clearField = useLabelFormatter(cadReplLabel("cad.repl.clearField"));
  const selected = useLabelFormatter(cadReplLabel("cad.repl.selected"));
  return {
    clear: useLabel(cadReplLabel("cad.repl.clear")),
    clearField: (field: string) => clearField({ field }),
    attributes: useLabel(cadReplLabel("cad.repl.attributes")),
    enabled: useLabel(cadReplLabel("cad.repl.enabled")),
    stats: useLabel(cadReplLabel("cad.repl.stats")),
    model: useLabel(cadReplLabel("cad.repl.model")),
    properties: useLabel(cadReplLabel("cad.repl.properties")),
    selection: useLabel(cadReplLabel("cad.repl.selection")),
    selected: (count: number) => selected({ count }),
    editor: useLabel(cadReplLabel("cad.repl.editor")),
    modelDefinition: useLabel(cadReplLabel("cad.repl.modelDefinition")),
    transferFrom: useLabel(cadReplLabel("cad.repl.transferFrom")),
    transferTo: useLabel(cadReplLabel("cad.repl.transferTo")),
    showPrimitives: useLabel(cadReplLabel("cad.repl.showPrimitives")),
    filterPrimitives: useLabel(cadReplLabel("cad.repl.filterPrimitives")),
    showTypologies: useLabel(cadReplLabel("cad.repl.showTypologies")),
    selectionTypologies: useLabel(cadReplLabel("cad.repl.selectionTypologies")),
    lastResponse: useLabel(cadReplLabel("cad.repl.lastResponse")),
    selectIncoming: useLabel(cadReplLabel("cad.repl.selectIncoming")),
    selectOutgoing: useLabel(cadReplLabel("cad.repl.selectOutgoing")),
  };
}
