/** 🌱 Catalog-resolved artifact creation kinds and the Space artifact creation host owner. */
import { createElement, useState, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { flushSync } from "react-dom";
import type { BackboneWorkerResponse } from "@semio-tech/framework-os";
import { UIDialog, uiI18n } from "@semio-tech/ui-react";
import { artifactKindChoiceDraftRetirementsV1, captureSpaceArtifactCreationCatalogAuthorityV1, spaceArtifactCreationCatalogRefreshRequestV1, spaceArtifactCreationOwnerAcceptsStatus, spaceArtifactCreationReadyOpening, spaceArtifactCreationRequestFromAction, selectedSpaceArtifactCreationCatalogV1, type SpaceArtifactCreationCatalogAuthorityV1, type SpaceArtifactCreationOwnerV1 } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import { EMPTY_APP_LABELS_OVERLAY, renderStagedArgControl, resolveDialogDefinition } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import deepEqual from "fast-deep-equal";
import dialogOriginFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🔣️.json";
import { type ShellDialogOriginV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🟦️.ts";
import artifactCreationProgressFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🌱️artifact-creation/🔣️.json";
import artifactCreationCatalogAuthorityFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🌱️artifact-creation/🪪️catalog-authority/🔣️.json";
import artifactCreationReadyOpeningFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🌱️artifact-creation/🚪️ready-opening/🔣️.json";
import { runArtifactCreationReadyOpeningV1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { runDocumentOpeningAttemptV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { ARTIFACT_CREATION_PROGRESS_CAPACITY, ARTIFACT_CREATION_PROGRESS_TEXT_V1, ArtifactCreationCatalogNotice, ArtifactCreationProgressNotice, artifactCreationProgressLocaleV1, artifactCreationProgressRoleV1, artifactCreationProgressTerminalV1, reduceArtifactCreationProgressUiV1, type ArtifactCreationProgressOwnerV1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🟦️.tsx";
import { OwnedShellDialog, type OwnedShellDialogProps } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx";
import { createInstance as createTranslationOracle } from "i18next";
import type { DialogDefinition } from "@semio-tech/framework";
import { unresolvedActionArgs } from "@semio-tech/framework";
import choiceFixture from "../../../../../../../🔨️modules/🧩️action-argument-resolution/🧪️tests/🧫️fixtures/🔽️choices/🔣️.json";
import { semioSchemaAjvV1 } from "../../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { type ActionArgDef } from "@semio-tech/framework";
import { type ResolvedActionArgDef } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { computeAccessibleName } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("catalog-resolved artifact creation kinds", () => {
  it("matches the independent enum validator for unresolved required and host-resolved choices", () => {
    const ajv = semioSchemaAjvV1({ strict: true });
    
    for (const row of choiceFixture.cases) {
      const def: ActionArgDef = {
        id: "kindChoice",
        label: "Kind",
        required: row.required,
        schema: {
          kind: "string",
          options: row.options.map((value) => ({ value, label: value })),
          ...(row.format === "text" ? {} : { format: row.format === "artifactKind" ? { kind: "artifactKind", roles: ["editor"] } : { kind: "surfaceApp", roles: ["editor"], dialectArg: "dialect" } }),
        },
      };
      const data = row.value === null || row.value === "" ? {} : { kindChoice: row.value };
      const property = row.options.length > 0 ? { enum: row.options } : row.format === "text" ? {} : false;
      const oracle = ajv.compile({ type: "object", required: row.required ? ["kindChoice"] : [], properties: { kindChoice: property } });
      expect(!oracle(data), row.id).toBe(row.unresolved);
      expect(unresolvedActionArgs([def], data), row.id).toEqual(row.unresolved ? ["kindChoice"] : []);
    }
  });
  const localized = (en: string, de: string) => ({ native: { en, de }, reuse: { en, de } });
  const dialog: DialogDefinition = {
    id: "createArtifact",
    title: localized("Create Artifact", "Artefakt erstellen"),
    args: [{ id: "kindChoice", label: localized("Kind", "Art"), required: true, schema: { kind: "string", options: [], format: { kind: "artifactKind", roles: ["editor"] } } }],
    submitAction: "createArtifact",
    submitLabel: localized("Create", "Erstellen"),
  };
  const manifests = [
    {
      pluginId: "neutral-host-fixture",
      label: "Fixture",
      version: "1",
      apps: [
        { id: "fixture-editor", role: "editor", dialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "*" }, label: localized("Editor", "Editor"), io: { artifactSchema: "fixture.counter" } },
        { id: "fixture-viewer", role: "viewer", dialect: { artifactKind: "fixture.other-owner.counter", standard: "1", subset: "*" }, label: localized("Viewer", "Betrachter"), io: { artifactSchema: "fixture.counter" } },
      ],
      artifactKinds: [{ schema: "fixture.counter", label: localized("Counter", "Zähler") }],
      workflows: [],
      examples: [],
    },
  ];

  it("keeps unavailable catalog feedback inside the dialog and requires a fresh selection after catalog withdrawal", async () => {
    for (const locale of ["en", "de"] as const) {
      await uiI18n.changeLanguage(locale);
      const ready = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", locale, manifests);
      const unavailable = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", locale, manifests, []);
      const submit = vi.fn();
      const kindLabel = locale === "en" ? "Kind" : "Art";
      const kindName = locale === "en" ? "Counter" : "Zähler";
      const submitLabel = locale === "en" ? "Create" : "Erstellen";
      const props = (definition: typeof ready, phase: "ready" | "loading" | "unavailable"): OwnedShellDialogProps<ResolvedActionArgDef> => ({
        owner: { openingId: 1, dialogId: dialog.id, origin: dialogOriginFixture.owner, seedArgs: { kindChoice: "forged" } },
        dialog: definition,
        isCurrent: () => true,
        close: () => true,
        dispatch: submit,
        notice: createElement(ArtifactCreationCatalogNotice, { status: { phase }, locale }),
        renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field),
      });
      const view = render(createElement(OwnedShellDialog<ResolvedActionArgDef>, props(ready, "ready")));
      try {
        expect(view.getByRole("button", { name: submitLabel }).hasAttribute("disabled")).toBe(true);
        for (const phase of choiceFixture.catalogTransitions as ("loading" | "unavailable")[]) {
          fireEvent.click(view.getByRole("combobox", { name: kindLabel }));
          fireEvent.click(view.getByRole("option", { name: kindName }));
          expect(view.getByRole("button", { name: submitLabel }).hasAttribute("disabled")).toBe(false);
          view.rerender(createElement(OwnedShellDialog<ResolvedActionArgDef>, props(unavailable, phase)));
          expect(view.getByRole("dialog").querySelector("input#kindChoice")).toBeNull();
          expect(view.getByRole("combobox", { name: kindLabel }).hasAttribute("disabled")).toBe(true);
          const notice = view.getByRole(phase === "unavailable" ? "alert" : "status");
          expect(view.getByRole("dialog").contains(notice)).toBe(true);
          expect(computeAccessibleName(notice)).toBe(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].catalog[phase]);
          expect(notice.closest('[inert],[aria-hidden="true"]')).toBeNull();
          const button = view.getByRole("button", { name: submitLabel });
          expect(button.hasAttribute("disabled")).toBe(true);
          fireEvent.click(button);
          fireEvent.keyDown(view.getByRole("dialog"), { key: "Enter", ctrlKey: true });
          expect(submit).not.toHaveBeenCalled();
          view.rerender(createElement(OwnedShellDialog<ResolvedActionArgDef>, props(ready, "ready")));
          expect(view.getByRole("button", { name: submitLabel }).hasAttribute("disabled")).toBe(true);
        }
        fireEvent.click(view.getByRole("combobox", { name: kindLabel }));
        fireEvent.click(view.getByRole("option", { name: kindName }));
        fireEvent.click(view.getByRole("button", { name: submitLabel }));
        expect(submit).toHaveBeenCalledTimes(1);
        expect(JSON.parse(submit.mock.calls[0]![2].kindChoice).kindId).toBe("fixture.neutral-host-fixture.counter");
      } finally {
        view.unmount();
      }
    }
  });

  it("retires only choices when their catalog generation changes, even if the same tuple returns", async () => {
    await uiI18n.changeLanguage("en");
    const resolved = resolveDialogDefinition(
      {
        ...dialog,
        args: [
          ...dialog.args,
          { id: "name", label: localized("Name", "Name"), required: true, schema: { kind: "string", options: [] } },
          { id: "color", label: localized("Color", "Farbe"), required: true, schema: { kind: "string", options: [{ value: "blue", label: localized("Blue", "Blau") }] } },
        ],
      },
      EMPTY_APP_LABELS_OVERLAY,
      "native",
      "en",
      manifests,
    );
    const props = {
      dialog: resolved,
      onSubmit: vi.fn(),
      onChoose: vi.fn(),
      onCancel: vi.fn(),
      renderField: (def: ResolvedActionArgDef, value: unknown, change: (value: unknown) => void, field: Parameters<OwnedShellDialogProps<ResolvedActionArgDef>["renderField"]>[3]) => renderStagedArgControl(def, value, change, false, field),
    };
    const view = render(createElement(UIDialog<ResolvedActionArgDef>, { ...props, choiceRevisions: { kindChoice: choiceFixture.generation.before } }));
    try {
      fireEvent.change(view.getByRole("textbox", { name: "Name" }), { target: { value: choiceFixture.generation.text } });
      fireEvent.click(view.getByRole("combobox", { name: "Color" }));
      fireEvent.click(view.getByRole("option", { name: "Blue" }));
      fireEvent.click(view.getByRole("combobox", { name: "Kind" }));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      view.rerender(createElement(UIDialog<ResolvedActionArgDef>, { ...props, choiceRevisions: { kindChoice: choiceFixture.generation.after } }));
      expect((view.getByRole("textbox", { name: "Name" }) as HTMLInputElement).value).toBe(choiceFixture.generation.text);
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(true);
      fireEvent.click(view.getByRole("combobox", { name: "Kind" }));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      fireEvent.click(view.getByRole("button", { name: "Create" }));
      expect(props.onSubmit).toHaveBeenCalledTimes(1);
      expect(props.onSubmit.mock.calls[0]![0].name).toBe(choiceFixture.generation.text);
      expect(props.onSubmit.mock.calls[0]![0].color).toBe(choiceFixture.generation.staticChoice);
    } finally {
      view.unmount();
    }
  });

  it("rejects an already queued submit handler when capture retires its catalog generation", async () => {
    await uiI18n.changeLanguage("en");
    const resolved = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests);
    const submit = vi.fn();
    function Harness(): ReactElement {
      const [revision, setRevision] = useState(choiceFixture.generation.before);
      return createElement(
        "div",
        { onKeyDownCapture: () => flushSync(() => setRevision(choiceFixture.generation.after)) },
        createElement(UIDialog<ResolvedActionArgDef>, {
          dialog: resolved,
          choiceRevisions: { kindChoice: revision },
          onSubmit: submit,
          onChoose: () => {},
          onCancel: () => {},
          renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field),
        }),
      );
    }
    const view = render(createElement(Harness));
    try {
      fireEvent.click(view.getByRole("combobox", { name: "Kind" }));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      fireEvent.keyDown(view.getByRole("dialog"), { key: "Enter", ctrlKey: true });
      expect(submit).toHaveBeenCalledTimes(choiceFixture.generation.staleSubmits);
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(true);
    } finally {
      view.unmount();
    }
  });

  it("names the actual staged kind picker and retains the chosen catalog tuple inside its modal", async () => {
    await uiI18n.changeLanguage("en");
    const resolved = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests);
    const submit = vi.fn();
    const cancel = vi.fn();
    const view = render(createElement(UIDialog<ResolvedActionArgDef>, { dialog: resolved, onSubmit: submit, onChoose: vi.fn(), onCancel: cancel, renderField: (def, value, change, field) => renderStagedArgControl(def, value, change, false, field) }));
    try {
      const picker = view.getByRole("combobox", { name: "Kind" });
      expect(computeAccessibleName(picker)).toBe("Kind");
      expect(picker.getAttribute("aria-required")).toBe("true");
      fireEvent.click(picker);
      expect(document.activeElement).toBe(view.getByRole("listbox"));
      fireEvent.click(view.getByRole("option", { name: "Counter" }));
      expect(view.getByRole("button", { name: "Create" }).hasAttribute("disabled")).toBe(false);
      fireEvent.click(view.getByRole("button", { name: "Create" }));
      expect(cancel).not.toHaveBeenCalled();
      expect(submit).toHaveBeenCalledTimes(1);
      expect(JSON.parse(submit.mock.calls[0]![0].kindChoice).kindId).toBe("fixture.neutral-host-fixture.counter");
    } finally {
      view.unmount();
    }
  });

  it("projects only live catalog editor kinds into the ordinary dialog in the requested locale", () => {
    const english = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests);
    const german = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "de", manifests);
    expect(english.args[0]?.schema).toMatchObject({ kind: "string", options: [{ label: "Counter" }] });
    expect(german.args[0]?.schema).toMatchObject({ kind: "string", options: [{ label: "Zähler" }] });
    const option = english.args[0]?.schema.kind === "string" ? english.args[0].schema.options[0] : undefined;
    expect(option?.value).toBe('{"kindId":"fixture.neutral-host-fixture.counter","schema":"fixture.counter","dialect":{"artifactKind":"fixture.neutral-host-fixture.counter","standard":"1","subset":"*"},"label":{"en":"Counter","de":"Zähler"}}');
    expect(JSON.parse(option?.value ?? "null").kindId).toBe("fixture.neutral-host-fixture.counter");
    expect(JSON.stringify(english)).not.toContain("fixture.other-owner.counter");
  });

  it("uses only the selected trusted catalog projection and fails closed while it is unavailable", () => {
    const selected = [{ kindId: "fixture.neutral-host-fixture.counter", schema: "fixture.neutral-host-fixture.counter", dialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "any" }, label: { en: "Shared Counter", de: "Gemeinsamer Zähler" } }];
    const exact = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "de", manifests, selected);
    const unavailable = resolveDialogDefinition(dialog, EMPTY_APP_LABELS_OVERLAY, "native", "en", manifests, []);
    expect(exact.args[0]?.schema).toMatchObject({ kind: "string", options: [{ label: "Gemeinsamer Zähler" }] });
    expect(JSON.stringify(exact)).not.toContain("fixture.other-owner.counter");
    expect(unavailable.args[0]?.schema).toMatchObject({ kind: "string", options: [] });
  });
});

describe("Space artifact creation host owner", () => {
  const requestId = "1".repeat(32);
  const choice = '{"kindId":"fixture.neutral-host-fixture.counter","schema":"fixture.neutral-host-fixture.counter","dialect":{"artifactKind":"fixture.neutral-host-fixture.counter","standard":"1","subset":"*"},"label":{"en":"Counter","de":"Zähler"}}';
  const catalogOrigin: ShellDialogOriginV1 = {
    pluginId: "space",
    appId: "space-editor",
    controllerId: "space",
    sessionInstanceId: 7,
    document: {
      runtimeKey: artifactCreationCatalogAuthorityFixture.catalog.runtimeKey,
      clientInstanceId: artifactCreationCatalogAuthorityFixture.catalog.clientInstanceId,
      scope: { spaceId: artifactCreationCatalogAuthorityFixture.catalog.spaceId, documentId: "index" },
    },
  };
  const catalog = {
    kind: "space-artifact-creation-catalog" as const,
    clientInstanceId: artifactCreationCatalogAuthorityFixture.catalog.clientInstanceId,
    spaceId: artifactCreationCatalogAuthorityFixture.catalog.spaceId,
    catalogGenerationId: artifactCreationCatalogAuthorityFixture.catalog.catalogGenerationId,
    kinds: [JSON.parse(choice)],
  };
  const catalogStatus = { kind: "space-artifact-creation-catalog-status" as const, clientInstanceId: catalog.clientInstanceId, spaceId: catalog.spaceId, phase: "ready" as const };
  const catalogAuthority = captureSpaceArtifactCreationCatalogAuthorityV1(catalog, catalogStatus, catalogOrigin)!;
  const owner: SpaceArtifactCreationOwnerV1 = {
    requestId,
    name: "Shared Map",
    spaceId: "space-a",
    expectedCatalogGenerationId: catalogAuthority.catalogGenerationId,
    kindId: "fixture.neutral-host-fixture.counter",
    runtimeKey: artifactCreationCatalogAuthorityFixture.catalog.runtimeKey,
    clientInstanceId: artifactCreationCatalogAuthorityFixture.catalog.clientInstanceId,
    sessionInstanceId: 7,
    opening: false,
    cancelRequested: false,
    ready: null,
  };

  it("forwards only the user intent and rejects presentation overposts or mismatched kinds", () => {
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice, name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toEqual({
      kind: "space-artifact-create",
      requestId,
      spaceId: "space-a",
      expectedCatalogGenerationId: catalogAuthority.catalogGenerationId,
      kindId: "fixture.neutral-host-fixture.counter",
      name: "Shared Map",
    });
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice, name: "Shared Map", documentId: "forged" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toBeNull();
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice.replaceAll("fixture.neutral-host-fixture.counter", "fixture.other-owner.counter"), name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toBeNull();
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice.replace('"kindId":"fixture.neutral-host-fixture.counter"', '"kindId":"forged"'), name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority)).toBeNull();
  });

  it("creates and opens a catalog member whose creation kind differs from the dialect that opens it", () => {
    const drawing = '{"kindId":"2d.drawing","schema":"drawing.document","dialect":{"artifactKind":"s.draw.drawing","standard":"1","subset":"*"},"label":{"en":"Editor","de":"Editor"}}';
    const drawingCatalog = { ...catalog, kinds: [JSON.parse(drawing)] };
    const drawingAuthority = captureSpaceArtifactCreationCatalogAuthorityV1(drawingCatalog, catalogStatus, catalogOrigin)!;
    expect(spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: drawing, name: "Plan" }, "space-a", requestId, drawingAuthority, drawingAuthority)).toEqual({
      kind: "space-artifact-create",
      requestId,
      spaceId: "space-a",
      expectedCatalogGenerationId: drawingAuthority.catalogGenerationId,
      kindId: "2d.drawing",
      name: "Plan",
    });
    expect(
      spaceArtifactCreationReadyOpening({
        kind: "space-artifact-creation-status",
        requestId,
        spaceId: "space-a",
        catalogGenerationId: drawingAuthority.catalogGenerationId,
        phase: "ready",
        ready: { artifactId: `artifact-${"6".repeat(32)}`, kindId: "2d.drawing", artifactSchema: "drawing.document", parentDialect: { artifactKind: "s.draw.drawing", standard: "1", subset: "*" } },
      }),
    ).toEqual({ artifactRef: "s.draw.drawing@1/*", artifactId: `artifact-${"6".repeat(32)}`, spaceId: "space-a", schema: "drawing.document" });
  });

  it("admits only an exact captured and live catalog generation member", () => {
    const withoutMember = { ...catalogAuthority, kindChoices: [] };
    const variants: Readonly<Record<string, SpaceArtifactCreationCatalogAuthorityV1 | null>> = {
      current: catalogAuthority,
      "without-member": withoutMember,
      unavailable: null,
      "rotated-generation": { ...catalogAuthority, catalogGenerationId: "4".repeat(64) },
      "replaced-client": { ...catalogAuthority, clientInstanceId: "22222222-2222-4222-8222-222222222222" },
      "other-space": { ...catalogAuthority, spaceId: "space-b" },
    };
    for (const row of artifactCreationCatalogAuthorityFixture.cases) {
      const selected = row.choice === "member" ? choice : choice.replaceAll("fixture.neutral-host-fixture.counter", "fixture.other-owner.counter");
      const request = spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: selected, name: "Shared Map" }, "space-a", requestId, variants[row.captured], variants[row.live]);
      expect(request !== null, row.id).toBe(row.admitted);
    }
    const ready = selectedSpaceArtifactCreationCatalogV1(catalog, catalogStatus, catalogOrigin)!;
    const withdrawn = selectedSpaceArtifactCreationCatalogV1(null, { ...catalogStatus, phase: "unavailable" }, catalogOrigin)!;
    expect(ready).toMatchObject({ phase: "ready", kinds: catalog.kinds, authority: catalogAuthority });
    expect(withdrawn).toMatchObject({ phase: "unavailable", kinds: [], authority: null });
    expect(ready.choiceRevision).not.toBe(withdrawn.choiceRevision);
  });

  it("binds creation acceptance and ready status to the selected catalog generation", () => {
    const request = spaceArtifactCreationRequestFromAction("os.create-space-artifact", { kindChoice: choice, name: "Shared Map" }, "space-a", requestId, catalogAuthority, catalogAuthority);
    const captured = { ...owner, expectedCatalogGenerationId: catalogAuthority.catalogGenerationId };
    const statuses = artifactCreationCatalogAuthorityFixture.statusCases.map((row) => {
      const message = {
        kind: "space-artifact-creation-status" as const,
        requestId,
        spaceId: owner.spaceId,
        ...(row.generation === "missing" ? {} : { catalogGenerationId: row.generation === "current" ? catalogAuthority.catalogGenerationId : "4".repeat(64) }),
        phase: "ready" as const,
        ready: { artifactId: `artifact-${"2".repeat(32)}`, kindId: owner.kindId, artifactSchema: "fixture.neutral-host-fixture.counter", parentDialect: { artifactKind: owner.kindId, standard: "1", subset: "*" } },
      };
      return spaceArtifactCreationOwnerAcceptsStatus(captured, message as Extract<BackboneWorkerResponse, { kind: "space-artifact-creation-status" }>);
    });
    expect({ requestGeneration: (request as unknown as Record<string, unknown>)?.expectedCatalogGenerationId, statuses }).toEqual({
      requestGeneration: catalogAuthority.catalogGenerationId,
      statuses: artifactCreationCatalogAuthorityFixture.statusCases.map((row) => row.admitted),
    });
  });

  it("refreshes only the exact current selected catalog after an owned initial conflict", () => {
    const refresh = { kind: "space-artifact-creation-catalog-refresh-required" as const, requestId, spaceId: owner.spaceId, catalogGenerationId: owner.expectedCatalogGenerationId };
    const ready = {
      kind: "space-artifact-creation-status" as const,
      requestId,
      spaceId: owner.spaceId,
      catalogGenerationId: owner.expectedCatalogGenerationId,
      phase: "ready" as const,
      ready: { artifactId: `artifact-${"2".repeat(32)}`, kindId: owner.kindId, artifactSchema: "fixture.neutral-host-fixture.counter", parentDialect: { artifactKind: owner.kindId, standard: "1", subset: "*" } },
    };
    const owners: Readonly<Record<string, SpaceArtifactCreationOwnerV1 | null>> = { current: owner, absent: null, ready: { ...owner, ready }, opening: { ...owner, opening: true } };
    const authorities: Readonly<Record<string, SpaceArtifactCreationCatalogAuthorityV1 | null>> = {
      current: catalogAuthority,
      unavailable: null,
      "rotated-generation": { ...catalogAuthority, catalogGenerationId: "4".repeat(64) },
      "replaced-runtime": { ...catalogAuthority, runtimeKey: "hub:space-a:replacement" },
    };
    const origins: Readonly<Record<string, ShellDialogOriginV1 | null>> = {
      current: catalogOrigin,
      "replaced-session": { ...catalogOrigin, sessionInstanceId: catalogOrigin.sessionInstanceId + 1 },
      "replaced-client": { ...catalogOrigin, document: { ...catalogOrigin.document!, clientInstanceId: "22222222-2222-4222-8222-222222222222" } },
      "other-document": { ...catalogOrigin, document: { ...catalogOrigin.document!, scope: { spaceId: owner.spaceId, documentId: "other" } } },
    };
    const messages: Readonly<Record<string, Extract<BackboneWorkerResponse, { kind: "space-artifact-creation-catalog-refresh-required" }>>> = {
      current: refresh,
      "replaced-request": { ...refresh, requestId: "2".repeat(32) },
      "rotated-generation": { ...refresh, catalogGenerationId: "4".repeat(64) },
    };
    for (const row of artifactCreationCatalogAuthorityFixture.refreshCases) {
      const request = spaceArtifactCreationCatalogRefreshRequestV1(owners[row.owner], authorities[row.authority], origins[row.origin], messages[row.message]);
      expect(request, row.id).toEqual(row.admitted ? { kind: "space-artifact-creation-catalog-open", clientInstanceId: owner.clientInstanceId, spaceId: owner.spaceId } : null);
    }
  });

  it("retires only staged artifact-kind fields on catalog revision", () => {
    const definitions = artifactCreationCatalogAuthorityFixture.draftCases.map((row) => ({ ownerId: row.id, args: [{ id: "value", artifactKind: row.format === "artifactKind" }] }));
    const staged = Object.fromEntries(artifactCreationCatalogAuthorityFixture.draftCases.map((row) => [row.id, { value: row.format }]));
    const retired = new Set(artifactKindChoiceDraftRetirementsV1(staged, definitions).map((row) => row.ownerId));
    for (const row of artifactCreationCatalogAuthorityFixture.draftCases) expect(retired.has(row.id), row.id).toBe(row.clear);
  });

  it("opens only an exact Ready tuple owned by the originating Space mount", () => {
    const ready = {
      kind: "space-artifact-creation-status" as const,
      requestId,
      spaceId: "space-a",
      catalogGenerationId: owner.expectedCatalogGenerationId,
      phase: "ready" as const,
      ready: {
        artifactId: `artifact-${"2".repeat(32)}`,
        kindId: "fixture.neutral-host-fixture.counter",
        artifactSchema: "fixture.neutral-host-fixture.counter",
        parentDialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "*" },
      },
    };
    expect(spaceArtifactCreationOwnerAcceptsStatus(owner, ready)).toBe(true);
    expect(spaceArtifactCreationOwnerAcceptsStatus(owner, { ...ready, requestId: "3".repeat(32) })).toBe(false);
    expect(spaceArtifactCreationOwnerAcceptsStatus(owner, { ...ready, ready: { ...ready.ready, kindId: "s.draw.draw" } })).toBe(false);
    expect(spaceArtifactCreationOwnerAcceptsStatus({ ...owner, ready }, ready)).toBe(true);
    expect(spaceArtifactCreationOwnerAcceptsStatus({ ...owner, ready }, { ...ready, ready: { ...ready.ready, artifactId: `artifact-${"5".repeat(32)}` } })).toBe(false);
    expect(spaceArtifactCreationReadyOpening(ready)).toEqual({
      artifactRef: "fixture.neutral-host-fixture.counter@1/*",
      artifactId: `artifact-${"2".repeat(32)}`,
      spaceId: "space-a",
      schema: "fixture.neutral-host-fixture.counter",
    });
    expect(spaceArtifactCreationReadyOpening({ kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId: owner.expectedCatalogGenerationId, phase: "preparing" })).toBeNull();
  });

  it("publishes only a committed current target and releases every private rejected target once", async () => {
    for (const row of artifactCreationReadyOpeningFixture.cases) {
      let currentIndex = 0;
      let releases = 0;
      let publishes = 0;
      let failures = 0;
      const outcome = await runArtifactCreationReadyOpeningV1({
        current: () => row.current[Math.min(currentIndex++, row.current.length - 1)]!,
        prepare: async () => {
          if (row.prepare === "unused") throw new Error("unexpected prepare");
          return row.prepare === "missing" ? null : { id: row.id };
        },
        open: async () => {
          if (row.open === "unused") throw new Error("unexpected open");
          if (row.open === "failed") throw new Error("attach failed");
          return row.open === "committed" ? { committed: true as const, runtimeKey: row.id, clientInstanceId: "client" } : null;
        },
        publish: () => {
          publishes += 1;
        },
        release: async () => {
          releases += 1;
          if (row.release === "failed") throw new Error("release failed");
        },
        failed: () => {
          failures += 1;
        },
      });
      expect(outcome, row.id).toBe(row.outcome);
      expect(releases, row.id).toBe(row.release === "unused" ? 0 : 1);
      expect(publishes, row.id).toBe(row.publish ? 1 : 0);
      expect(failures, row.id).toBe(row.failure ? 1 : 0);
    }
  });

  it("requires an explicit same-generation mount after the document port becomes ready", async () => {
    const { createArtifactCreationCatalogMountV1 } = await import("../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts");
    for (const row of artifactCreationReadyOpeningFixture.mountCases) {
      const gate = createArtifactCreationCatalogMountV1(catalogAuthority.catalogGenerationId);
      let outcome = "pending";
      const settled = gate.ready.then(
        () => {
          outcome = "ready";
        },
        () => {
          outcome = "failed";
        },
      );
      await Promise.resolve();
      expect(outcome, row.id).toBe("pending");
      expect(gate.current(), row.id).toBe(true);
      const accepted: boolean[] = [];
      for (const action of row.actions) {
        if (action === "close") gate.close(new Error("document closed"));
        else accepted.push(gate.accept(action === "current" ? catalogAuthority.catalogGenerationId : action === "rotated" ? "4".repeat(64) : ""));
      }
      await settled;
      const expected = { accepted: row.accepted, outcome: row.outcome };
      expect({ accepted, outcome }, row.id).toEqual(expected);
      expect(deepEqual({ accepted, outcome }, expected), row.id).toBe(true);
      expect(gate.current(), row.id).toBe(row.committed);
    }
    for (const generation of ["", "0".repeat(64), "A".repeat(64), "1".repeat(63), "1".repeat(65), null, ["1".repeat(64)]]) expect(() => createArtifactCreationCatalogMountV1(generation as string)).toThrow();
  });

  it("commits private document opening only after a still-current creation mount", async () => {
    const { createArtifactCreationCatalogMountV1 } = await import("../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts");
    for (const row of artifactCreationReadyOpeningFixture.mountCases) {
      const gate = createArtifactCreationCatalogMountV1(catalogAuthority.catalogGenerationId);
      let commits = 0,
        closes = 0,
        detaches = 0,
        retires = 0;
      let failure: string | null = null;
      let attached!: () => void;
      const attachment = new Promise<void>((resolve) => {
        attached = resolve;
      });
      const documentReady = row.id === "rebootstrap-before-port-ready" ? new Promise<void>(() => {}) : Promise.resolve();
      const opening = runDocumentOpeningAttemptV1({
        deadlineMs: 1_000,
        current: gate.current,
        socket: async () => {},
        attach: async () => {
          attached();
          await gate.attach(documentReady);
        },
        commit: () => {
          commits += 1;
        },
        close: () => {
          closes += 1;
          gate.close(new Error("opening closed"));
        },
        detach: async () => {
          detaches += 1;
        },
        retire: () => {
          retires += 1;
        },
      }).catch((error) => {
        failure = error instanceof Error ? error.message : String(error);
        return false;
      });
      await attachment;
      expect({ commits, closes, detaches, retires }, row.id).toEqual({ commits: 0, closes: 0, detaches: 0, retires: 0 });
      for (const action of row.actions) {
        if (action === "close") gate.close(new Error("document closed"));
        else gate.accept(action === "current" ? catalogAuthority.catalogGenerationId : action === "rotated" ? "4".repeat(64) : "");
      }
      const result = { committed: await opening, commits, closes, detaches, retires };
      const expected = { committed: row.committed, commits: Number(row.committed), closes: Number(!row.committed), detaches: Number(!row.committed), retires: 1 };
      expect(result, row.id).toEqual(expected);
      expect(deepEqual(result, expected), row.id).toBe(true);
      if (row.outcome === "failed") expect(failure, row.id).toBe(row.actions.includes("close") ? "document closed" : "artifact-creation.catalog-generation-mismatch");
    }
  });

  it("retains exact sibling owners through one-shot cancellation and accepts a racing Ready", () => {
    const progressOwner: ArtifactCreationProgressOwnerV1 = owner;
    const sibling: ArtifactCreationProgressOwnerV1 = { ...progressOwner, requestId: "2".repeat(32), name: "Second Map" };
    let state = reduceArtifactCreationProgressUiV1({}, { kind: "issued", owner: progressOwner, atMs: 0 });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: sibling, atMs: 0 });
    const unchanged = state;
    expect(
      reduceArtifactCreationProgressUiV1(state, { kind: "status", message: { kind: "space-artifact-creation-status", requestId: "3".repeat(32), spaceId: "space-a", catalogGenerationId: owner.expectedCatalogGenerationId, phase: "failed" } }),
    ).toBe(unchanged);
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "status", message: { kind: "space-artifact-creation-status", requestId, spaceId: "foreign-space", catalogGenerationId: owner.expectedCatalogGenerationId, phase: "failed" } })).toBe(
      unchanged,
    );
    state = reduceArtifactCreationProgressUiV1(state, { kind: "cancel-requested", requestId, spaceId: "space-a" });
    expect(state[requestId]).toMatchObject({ phase: "accepted", cancelRequested: true });
    expect(state[sibling.requestId]).toMatchObject({ phase: "accepted", cancelRequested: false });
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "cancel-requested", requestId, spaceId: "space-a" })).toBe(state);
    const wrongKind = {
      kind: "space-artifact-creation-status" as const,
      requestId,
      spaceId: "space-a",
      catalogGenerationId: owner.expectedCatalogGenerationId,
      phase: "ready" as const,
      ready: { artifactId: `artifact-${"4".repeat(32)}`, kindId: "s.draw.draw", artifactSchema: "s.draw.draw", parentDialect: { artifactKind: "s.draw.draw", standard: "1", subset: "*" } },
    };
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "status", message: wrongKind })).toBe(state);
    const ready = { ...wrongKind, ready: { ...wrongKind.ready, kindId: owner.kindId, artifactSchema: owner.kindId, parentDialect: { artifactKind: owner.kindId, standard: "1", subset: "*" } } };
    state = reduceArtifactCreationProgressUiV1(state, { kind: "status", message: ready });
    expect(state[requestId]).toMatchObject({ phase: "ready", cancelRequested: true, openingDisposition: "idle" });
    expect(state[sibling.requestId]).toMatchObject({ phase: "accepted", cancelRequested: false });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "open-failed", requestId, spaceId: "space-a" });
    expect(state[requestId]).toMatchObject({ phase: "ready", openingDisposition: "failed" });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "opening", requestId, spaceId: "space-a" });
    expect(state[requestId]).toMatchObject({ phase: "ready", openingDisposition: "opening" });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "cleared", requestId });
    expect(state[requestId]).toBeUndefined();
    expect(state[sibling.requestId]).toBeDefined();
  });

  it("bounds presentation state without evicting a live retained owner", () => {
    let state: ReturnType<typeof reduceArtifactCreationProgressUiV1> = {};
    const owners = Array.from({ length: ARTIFACT_CREATION_PROGRESS_CAPACITY }, (_, index) => ({
      ...owner,
      requestId: index.toString(16).padStart(32, "0"),
      name: `Map ${index}`,
    }));
    for (const candidate of owners) state = reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: candidate, atMs: 0 });
    const full = state;
    const replacement = { ...owner, requestId: "f".repeat(32), name: "Replacement" };
    expect(reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: replacement, atMs: 0 })).toBe(full);
    state = reduceArtifactCreationProgressUiV1(state, {
      kind: "status",
      message: { kind: "space-artifact-creation-status", requestId: owners[0]!.requestId, spaceId: owner.spaceId, catalogGenerationId: owner.expectedCatalogGenerationId, phase: "failed" },
    });
    state = reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: replacement, atMs: 0 });
    expect(Object.keys(state)).toHaveLength(ARTIFACT_CREATION_PROGRESS_CAPACITY);
    expect(state[owners[0]!.requestId]).toBeUndefined();
    expect(state[replacement.requestId]).toMatchObject({ phase: "accepted", cancelRequested: false });
  });

  it("renders the English and German lifecycle without inventing numeric progress", async () => {
    expect(deepEqual(artifactCreationProgressFixture.locales, ARTIFACT_CREATION_PROGRESS_TEXT_V1)).toBe(true);
    const oracle = createTranslationOracle();
    await oracle.init({
      fallbackLng: false,
      resources: {
        en: { translation: artifactCreationProgressFixture.locales.en },
        de: { translation: artifactCreationProgressFixture.locales.de },
      },
    });
    for (const row of artifactCreationProgressFixture.cases) {
      const phase = row.phase as keyof typeof ARTIFACT_CREATION_PROGRESS_TEXT_V1.en.phases;
      const locale = row.locale as "en" | "de";
      expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].phases[phase], row.id).toBe(oracle.t(`phases.${phase}`, { lng: locale }));
      expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.failed, row.id).toBe(oracle.t("opening.failed", { lng: locale }));
      expect(artifactCreationProgressRoleV1(phase, row.openingDisposition as "idle" | "opening" | "failed"), row.id).toBe(row.role);
      expect(artifactCreationProgressTerminalV1(phase), row.id).toBe(["ready", "indeterminate", "failed", "cancelled"].includes(phase));
      const view = render(
        createElement(ArtifactCreationProgressNotice, {
          state: { ...owner, issuedAtMs: Date.now(), phase, cancelRequested: row.cancelRequested, openingDisposition: row.openingDisposition as "idle" | "opening" | "failed" },
          locale,
          onCancel: () => {},
          onOpen: () => {},
        }),
      );
      const region = view.getByRole(row.role, { name: `${artifactCreationProgressFixture.locales[locale].heading}: Shared Map` });
      expect(computeAccessibleName(region), row.id).toBe(`${artifactCreationProgressFixture.locales[locale].heading}: Shared Map`);
      expect(region.getAttribute("aria-live"), row.id).toBe(row.live);
      expect(region.querySelector("progress"), row.id).toBeNull();
      const button = region.querySelector("button");
      if (row.retryable) expect(button?.textContent, row.id).toBe(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.retry);
      else if (artifactCreationProgressTerminalV1(phase)) expect(button, row.id).toBeNull();
      else expect(button?.hasAttribute("disabled"), row.id).toBe(row.cancelRequested);
      view.unmount();
    }
    for (const row of artifactCreationProgressFixture.catalogCases) {
      const locale = row.locale as "en" | "de";
      const phase = row.phase as "loading" | "ready" | "unavailable";
      const textKey = row.effectivePhase as "loading" | "ready" | "unavailable";
      expect(ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].catalog[textKey], row.id).toBe(oracle.t(`catalog.${textKey}`, { lng: locale }));
      const view = render(
        createElement(ArtifactCreationCatalogNotice, {
          status: { phase },
          hasChoices: row.hasChoices,
          locale,
        }),
      );
      const region = view.getByRole(row.role, { name: artifactCreationProgressFixture.locales[locale].catalog[textKey] });
      expect(region.getAttribute("aria-live"), row.id).toBe(row.live);
      expect(region.getAttribute("aria-busy"), row.id).toBe(textKey === "loading" ? "true" : null);
      expect(region.getAttribute("data-semio-artifact-creation-catalog"), row.id).toBe(textKey);
      view.unmount();
    }
    for (const locale of artifactCreationProgressFixture.unsupportedLocales) {
      expect(artifactCreationProgressLocaleV1(locale), locale).toBeNull();
      expect(
        renderToStaticMarkup(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "accepted", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} })),
        locale,
      ).toBe("");
    }
  });

  // 🐢️ ticket 26/09/23 S15: a creation the hub keeps working on says how long, in the person's language, and keeps its Cancel.
  it("tells how long the hub has been working on a running creation and keeps the Cancel, en and de", () => {
    for (const [locale, elapsed] of [
      ["en", "2 min 5 s"],
      ["de", "2 Min. 5 s"],
    ] as const) {
      const view = render(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now() - 125_400, phase: "accepted", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} }));
      const region = view.getByRole("status");
      expect(region.querySelector("[data-semio-artifact-creation-elapsed]")?.textContent).toBe(artifactCreationProgressFixture.locales[locale].waiting.replace("{elapsed}", elapsed));
      expect(view.getByRole("button", { name: `${artifactCreationProgressFixture.locales[locale].cancel}: Shared Map` }).hasAttribute("disabled")).toBe(false);
      view.unmount();
      const fresh = render(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "accepted", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} }));
      expect(fresh.container.querySelector("[data-semio-artifact-creation-elapsed]"), "no waiting line before the threshold").toBeNull();
      fresh.unmount();
      const done = render(
        createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now() - 125_400, phase: "indeterminate", cancelRequested: false, openingDisposition: "idle" }, locale, onCancel: () => {}, onOpen: () => {} }),
      );
      expect(done.container.querySelector("[data-semio-artifact-creation-elapsed]"), "a concluded creation shows no waiting line").toBeNull();
      done.unmount();
    }
  });

  it("dispatches cancel once with the exact retained owner", () => {
    const onCancel = vi.fn();
    const view = render(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "preparing", cancelRequested: false, openingDisposition: "idle" }, locale: "de", onCancel, onOpen: () => {} }));
    fireEvent.click(view.getByRole("button", { name: "Erstellung abbrechen: Shared Map" }));
    expect(onCancel).toHaveBeenCalledExactlyOnceWith(requestId, "space-a");
    view.rerender(createElement(ArtifactCreationProgressNotice, { state: { ...owner, issuedAtMs: Date.now(), phase: "preparing", cancelRequested: true, openingDisposition: "idle" }, locale: "de", onCancel, onOpen: () => {} }));
    fireEvent.click(view.getByRole("button", { name: "Erstellung abbrechen: Shared Map" }));
    expect(onCancel).toHaveBeenCalledTimes(1);
    view.unmount();
  });

  it("offers a bilingual open retry without issuing creation or cancellation again", () => {
    for (const locale of ["en", "de"] as const) {
      const onCancel = vi.fn();
      const onOpen = vi.fn();
      const view = render(
        createElement(ArtifactCreationProgressNotice, {
          state: { ...owner, issuedAtMs: Date.now(), phase: "ready", cancelRequested: false, openingDisposition: "failed" },
          locale,
          onCancel,
          onOpen,
        }),
      );
      fireEvent.click(view.getByRole("button", { name: `${ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.retry}: Shared Map` }));
      expect(onOpen).toHaveBeenCalledExactlyOnceWith(requestId);
      expect(onCancel).not.toHaveBeenCalled();
      view.unmount();
    }
  });
});
