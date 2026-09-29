# -*- coding: utf-8 -*-
"""S18 §14c (LW1's 8 engine-contract reds after T14's `kind-choices`): test-only (rule 22). The creation-kind laws' manifest
fixture predates the declared-kind contract — its apps name `io.documentSchema` (the resolver reads `io.artifactSchema`) and
declare no artifact kind, so T14's resolver (a package's DECLARED kinds only, labelled by the KIND) rightly offers nothing. The
fixture now declares the kind (`gis.map`, "GIS Map"/"GIS-Karte") on the manifest, names `io.artifactSchema`, and labels its apps
as the SDK does ("Editor"/"Viewer") — so the laws prove the kind's label wins over the app's; the expected encoded choice
carries `schema`. The three other reds (multiple dialogs / comboboxes, an unfired picker) were cascades: "names the actual
staged kind picker" unmounted only on success, leaving its modal behind; it now unmounts in `finally`. Idempotent."""
import pathlib

LAW = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts")

EDITS = [
    ('''      apps: [
        { id: "gis-map-editor", role: "editor", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "*" }, label: localized("GIS Map", "GIS-Karte"), io: { documentSchema: "gis.map" } },
        { id: "gis-map-viewer", role: "viewer", dialect: { artifactKind: "s.gis.viewer", standard: "1", subset: "*" }, label: localized("GIS Viewer", "GIS-Betrachter"), io: { documentSchema: "gis.map" } },
      ],
      workflows: [],
      examples: [],
    },''',
     '''      apps: [
        { id: "gis-map-editor", role: "editor", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "*" }, label: localized("Editor", "Editor"), io: { artifactSchema: "gis.map" } },
        { id: "gis-map-viewer", role: "viewer", dialect: { artifactKind: "s.gis.viewer", standard: "1", subset: "*" }, label: localized("Viewer", "Betrachter"), io: { artifactSchema: "gis.map" } },
      ],
      artifactKinds: [{ schema: "gis.map", label: localized("GIS Map", "GIS-Karte") }],
      workflows: [],
      examples: [],
    },'''),
    ("""    expect(option?.value).toBe('{"kindId":"s.gis.gismap","dialect":{"artifactKind":"s.gis.gismap","standard":"1","subset":"*"},"label":{"en":"GIS Map","de":"GIS-Karte"}}');""",
     """    expect(option?.value).toBe('{"kindId":"s.gis.gismap","schema":"gis.map","dialect":{"artifactKind":"s.gis.gismap","standard":"1","subset":"*"},"label":{"en":"GIS Map","de":"GIS-Karte"}}');"""),
    ('''    const view = render(createElement(UIDialog<ResolvedActionArgDef>, { dialog: resolved, onSubmit: submit, onCancel: cancel, renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field) }));
    const picker = view.getByRole("combobox", { name: "Kind" });
    expect(computeAccessibleName(picker)).toBe("Kind");
    expect(picker.getAttribute("aria-required")).toBe("true");
    fireEvent.click(picker);
    expect(document.activeElement).toBe(view.getByRole("listbox"));
    fireEvent.click(view.getByRole("option", { name: "GIS Map" }));
    expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
    fireEvent.click(view.getByRole("button", { name: "Create" }));
    expect(cancel).not.toHaveBeenCalled();
    expect(submit).toHaveBeenCalledTimes(1);
    expect(JSON.parse(submit.mock.calls[0]![0].kindChoice).kindId).toBe("s.gis.gismap");
    view.unmount();
  });''',
     '''    const view = render(createElement(UIDialog<ResolvedActionArgDef>, { dialog: resolved, onSubmit: submit, onCancel: cancel, renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field) }));
    try {
      const picker = view.getByRole("combobox", { name: "Kind" });
      expect(computeAccessibleName(picker)).toBe("Kind");
      expect(picker.getAttribute("aria-required")).toBe("true");
      fireEvent.click(picker);
      expect(document.activeElement).toBe(view.getByRole("listbox"));
      fireEvent.click(view.getByRole("option", { name: "GIS Map" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      fireEvent.click(view.getByRole("button", { name: "Create" }));
      expect(cancel).not.toHaveBeenCalled();
      expect(submit).toHaveBeenCalledTimes(1);
      expect(JSON.parse(submit.mock.calls[0]![0].kindChoice).kindId).toBe("s.gis.gismap");
    } finally {
      view.unmount();
    }
  });'''),
]


def main() -> None:
    text = LAW.read_text(encoding="utf-8")
    for old, new in EDITS:
        if new in text:
            continue
        assert text.count(old) == 1, old[:90]
        text = text.replace(old, new)
    LAW.write_text(text, encoding="utf-8")
    print("ok")


main()
