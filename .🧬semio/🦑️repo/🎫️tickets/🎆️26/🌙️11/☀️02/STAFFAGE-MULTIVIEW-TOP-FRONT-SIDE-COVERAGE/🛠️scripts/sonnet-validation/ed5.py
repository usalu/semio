import sys
p = sys.argv[1]
s = open(p, encoding="utf8").read()
a = s.index("function renderSubject(")
b = s.index("const CONVENTIONAL_NOTE")
new = '''const shapeGeometry = (scene: DrawingScene): string => JSON.stringify(scene.layers.flatMap((layer) => layer.items.flatMap((item) => (item.kind === "path" && item.id.startsWith("shape:") ? [[item.id, item.commands]] : []))));

const hasCue = (scene: DrawingScene): boolean => scene.layers.some((layer) => layer.items.some((item) => item.kind === "path" && (item.fill?.pattern !== undefined || (item.stroke?.dashMm?.length ?? 0) > 0)));

function renderSubject(asset: Asset, subject: Subject, source: DrawingSource, collector: Collector, symbol?: SymbolRepresentation): void {
  let geometryDone = false;
  const english = new Map<string, string>();
  for (const language of subject.languages)
    for (const mode of subject.modes) {
      let multi: MultiView;
      try {
        multi = resolveViews(asset, subject.query, source, mode, drawOptions(mode, source.denominator, false, language));
      } catch (error) {
        collector.checked(VIEWPOINT_LIST, [mode]);
        collector.fail(VIEWPOINT_LIST, [mode], "resolve-error", `${subject.id} ${language}`, messageOf(error));
        continue;
      }
      if (multi.state !== null && !geometryDone && symbol === undefined) {
        checkGeometry(asset, subject, multi.state, collector);
        geometryDone = true;
      }
      for (const view of VIEWPOINT_LIST) {
        const drawing = multi.views[view];
        collector.checked([view], [mode]);
        if (drawing.kind === "unavailable") {
          collector.fail([view], [mode], `unavailable-${drawing.code}`, subject.id, drawing.reason[language]);
          continue;
        }
        if (drawing.state !== multi.state) collector.fail([view], [mode], "state-identity", subject.id, "the view was projected from another evaluated state");
        if (symbol !== undefined && drawing.fidelity === "conventional-symbol" && !CONVENTIONAL_NOTE.test(`${symbol.dimensionalNote.en} ${symbol.dimensionalNote.de}`)) collector.fail([view], [mode], "symbol-conventional-note", subject.id, "a conventional symbol does not say that it is a conventional sign");
        checkScene(asset, subject, drawing.state, view, mode, drawing.scene, source.denominator, collector);
        if ((mode === "thermal" || mode === "forces" || mode === "moments") && !hasCue(drawing.scene)) collector.fail([view], [mode], "analysis-cue", subject.id, `the ${mode} scene has neither a hatch pattern nor a dashed stroke under default options`);
        const key = `${view}|${mode}`;
        if (language === "en") english.set(key, shapeGeometry(drawing.scene));
        else if (english.get(key) !== undefined && english.get(key) !== shapeGeometry(drawing.scene)) collector.fail([view], [mode], "language-geometry", subject.id, `the ${language} drawing has other shape geometry than the English one`);
      }
    }
}

'''
s = s[:a] + new + s[b:]
open(p, "w", encoding="utf8").write(s)
