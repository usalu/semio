/** 📋️ The selection attribute, model stats and selection property panes hosted by the REPL.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  Button,
  borderNormalClass,
  Checkbox,
  cn,
  Input,
  reactHostPort,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Fragment,
  type ReactNode
} from "@semio-tech/framework/ui/targets/react";
// #endregion 🔌️Adapters
import { type SpatialObjectRecord } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { type AttributeDefinitionSpec } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🎨️typology/🟦️.ts";
import { computeStat, formatStatOutputValue, listStatDefinitionsForModelDefinition, objectsForStatCompute, statDefinitionAppliesToScope } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/📊️stats/🟦️.ts";
import { Model } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧊️model/🟦️.ts";
import { listAttributeDefinitionsForModelDefinitionEntity, attributeDefinitionEditorKind, attributeDefinitionValueOptions, validateAttributeValue } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧭️definition-ownership/🟦️.ts";
import { derivePropertyValue, listApplicablePropertyDefinitionsForModelDefinition } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧾️primitive-document/🟦️.ts";
import { type SelectionTarget } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🪪️selection/🟦️.ts";
import { type SpatialKernel } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import { primaryAttributeSelectionTarget } from "../../🎬️actions/🟦️.ts";
import { cadFieldClass, ENUM_FIELD_NONE_VALUE } from "../🎨️style/🟦️.ts";

// #region 🪩️Repl
export interface SelectionAttributesPaneProps {
  readonly model: Model;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly selectionCount?: number;
  readonly onModelChange: (model: Model) => void;
}

/** 🏷️ Edits {@link Model.metadata} fields for the primary selection using active model-definition attribute assets. */
export function SelectionAttributesPane({ model, activeModelDefinitionId, selection, selectionCount, onModelChange }: SelectionAttributesPaneProps): ReactNode {
  const target = reactHostPort.useMemo(() => primaryAttributeSelectionTarget(selection), [selection]);
  const definitions = reactHostPort.useMemo(() => (target ? listAttributeDefinitionsForModelDefinitionEntity(activeModelDefinitionId, target.kind) : []), [activeModelDefinitionId, target]);
  if (!target) {
    return (
      <p className="text-muted-foreground text-xs leading-snug">
        Select a primitive or object to edit attributes for <code className="text-foreground">{activeModelDefinitionId}</code>.
      </p>
    );
  }
  if (!definitions.length) {
    return (
      <p className="text-muted-foreground text-xs leading-snug">
        No attribute definitions for <code className="text-foreground">{target.kind}</code> on this model definition.
      </p>
    );
  }
  const fields = model.metadata.get(target.id) ?? {};
  const count = selectionCount ?? selection.length;
  const setField = (defn: AttributeDefinitionSpec, value: unknown) => {
    if (!validateAttributeValue(defn, value)) return;
    model.metadata.setField(target.id, defn.field, value);
    onModelChange(model);
  };
  const clearField = (defn: AttributeDefinitionSpec) => {
    model.metadata.deleteField(target.id, defn.field);
    onModelChange(model);
  };
  const fieldRow = (defn: AttributeDefinitionSpec, current: unknown, control: ReactNode) => (
    <div key={defn.id} className="flex flex-col gap-half">
      <div className="flex items-center justify-between gap-single">
        <span className="text-xs font-medium">{defn.label}</span>
        {current !== undefined ? (
          <Button id={`cad.attribute.${defn.id}.clear`} type="button" variant="outline" icon="x" className="h-auto px-half py-0 text-2xs" onClick={() => clearField(defn)}>
            Clear
          </Button>
        ) : null}
      </div>
      {control}
    </div>
  );
  return (
    <div className="flex flex-col gap-single text-xs">
      <span className="text-foreground text-sm font-semibold">Attributes</span>
      <span className="text-muted-foreground text-2xs">
        {target.kind} · <code className="text-foreground">{target.id}</code>
        {count > 1 ? ` · ${count} selected` : ""}
      </span>
      {definitions.map((defn) => {
        const editor = attributeDefinitionEditorKind(defn);
        const current = fields[defn.field];
        if (editor === "enum") {
          const options = attributeDefinitionValueOptions(defn) ?? [];
          return fieldRow(
            defn,
            current,
            <Select
              id={`cad.attribute.${defn.id}`}
              value={typeof current === "string" ? current : ENUM_FIELD_NONE_VALUE}
              onValueChange={(value) => {
                if (value === ENUM_FIELD_NONE_VALUE) clearField(defn);
                else setField(defn, value);
              }}
            >
              <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={ENUM_FIELD_NONE_VALUE}>—</SelectItem>
                {options.map((option) => (
                  <SelectItem key={option} value={option}>
                    {option}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>,
          );
        }
        if (editor === "number") {
          return fieldRow(
            defn,
            current,
            <Input
              id={`cad.attribute.${defn.id}`}
              type="number"
              className={cadFieldClass}
              value={typeof current === "number" ? current : ""}
              onChange={(e) => {
                if (e.target.value === "") clearField(defn);
                else setField(defn, Number(e.target.value));
              }}
            />,
          );
        }
        if (editor === "boolean") {
          return fieldRow(
            defn,
            current,
            <label className="flex items-center gap-single">
              <Checkbox checked={current === true} onChange={(event) => setField(defn, event.target.checked)} />
              <span>Enabled</span>
            </label>,
          );
        }
        return fieldRow(
          defn,
          current,
          <Input
            id={`cad.attribute.${defn.id}`}
            type="text"
            className={cadFieldClass}
            value={typeof current === "string" ? current : current === undefined || current === null ? "" : JSON.stringify(current)}
            onChange={(e) => {
              if (!e.target.value) clearField(defn);
              else setField(defn, e.target.value);
            }}
          />,
        );
      })}
    </div>
  );
}

export interface SelectionPropertiesPaneProps {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly selectionCount?: number;
}

/** 📐️ Displays derived property values for the primary object selection using scoped property definitions. */
export interface ModelStatsPaneProps {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly selectionCount?: number;
}

/** 📊️ Displays live model-definition stats for whole-model and selection scopes. */
export function ModelStatsPane({ model, kernel, activeModelDefinitionId, selection, selectionCount }: ModelStatsPaneProps): ReactNode {
  const definitions = reactHostPort.useMemo(() => listStatDefinitionsForModelDefinition(activeModelDefinitionId), [activeModelDefinitionId]);
  const selectionObjects = reactHostPort.useMemo(() => {
    const objectTargets = selection.filter((row) => row.kind === "object");
    return objectTargets.map((row) => model.objects[row.id]).filter((row): row is SpatialObjectRecord => row !== undefined);
  }, [model, selection]);
  const [values, setValues] = reactHostPort.useState<Readonly<Record<string, Readonly<Record<string, Record<string, number>>>>>>({});
  reactHostPort.useEffect(() => {
    if (!definitions.length) {
      setValues({});
      return;
    }
    let cancelled = false;
    void (async () => {
      const next: Record<string, Record<string, Record<string, number>>> = {};
      for (const defn of definitions) {
        const scoped: Record<string, Record<string, number>> = {};
        for (const scope of ["model", "selection"] as const) {
          if (!statDefinitionAppliesToScope(defn, scope)) continue;
          if (scope === "selection" && selectionObjects.length === 0) continue;
          const objects = objectsForStatCompute(model, activeModelDefinitionId, defn, scope, selectionObjects);
          scoped[scope] = await computeStat(defn, { model, kernel, modelDefinitionId: activeModelDefinitionId, scope, objects });
        }
        next[defn.id] = scoped;
      }
      if (!cancelled) setValues(next);
    })();
    return () => {
      cancelled = true;
    };
  }, [activeModelDefinitionId, definitions, kernel, model, selectionObjects]);
  if (!definitions.length) return null;
  const count = selectionCount ?? selection.length;
  return (
    <div className="flex flex-col gap-single text-xs">
      <span className="text-foreground text-sm font-semibold">Stats</span>
      {definitions.map((defn) => (
        <div key={defn.id} className="flex flex-col gap-half">
          <span className="font-medium">{defn.label}</span>
          {statDefinitionAppliesToScope(defn, "model") ? (
            <div className="flex flex-col gap-half">
              <span className="text-muted-foreground text-2xs">Model</span>
              <div className="grid grid-cols-[minmax(0,1fr)_auto] gap-x-single gap-y-half">
                {defn.outputs.map((output) => (
                  <Fragment key={`${defn.id}:model:${output.key}`}>
                    <span>{output.label}</span>
                    <span className="text-element text-right tabular-nums">
                      {formatStatOutputValue(values[defn.id]?.model?.[output.key] ?? 0, output.format)}
                      {output.unit ? ` ${output.unit}` : ""}
                    </span>
                  </Fragment>
                ))}
              </div>
            </div>
          ) : null}
          {statDefinitionAppliesToScope(defn, "selection") && selectionObjects.length > 0 ? (
            <div className="flex flex-col gap-half">
              <span className="text-muted-foreground text-2xs">Selection{count > 1 ? ` · ${count} selected` : ""}</span>
              <div className="grid grid-cols-[minmax(0,1fr)_auto] gap-x-single gap-y-half">
                {defn.outputs.map((output) => (
                  <Fragment key={`${defn.id}:selection:${output.key}`}>
                    <span>{output.label}</span>
                    <span className="text-element text-right tabular-nums">
                      {formatStatOutputValue(values[defn.id]?.selection?.[output.key] ?? 0, output.format)}
                      {output.unit ? ` ${output.unit}` : ""}
                    </span>
                  </Fragment>
                ))}
              </div>
            </div>
          ) : null}
        </div>
      ))}
    </div>
  );
}

export function SelectionPropertiesPane({ model, kernel, activeModelDefinitionId, selection, selectionCount }: SelectionPropertiesPaneProps): ReactNode {
  const objectRow = reactHostPort.useMemo(() => {
    const objectTarget = selection.find((row) => row.kind === "object");
    return objectTarget ? (model.objects[objectTarget.id] ?? null) : null;
  }, [model, selection]);
  const definitions = reactHostPort.useMemo(() => (objectRow ? listApplicablePropertyDefinitionsForModelDefinition(activeModelDefinitionId, model, objectRow) : []), [activeModelDefinitionId, model, objectRow]);
  const [values, setValues] = reactHostPort.useState<Readonly<Record<string, Record<string, unknown>>>>({});
  reactHostPort.useEffect(() => {
    if (!objectRow || !definitions.length) {
      setValues({});
      return;
    }
    let cancelled = false;
    void (async () => {
      const next: Record<string, Record<string, unknown>> = {};
      for (const defn of definitions) {
        next[defn.id] = await derivePropertyValue(defn, { model, kernel, object: objectRow });
      }
      if (!cancelled) setValues(next);
    })();
    return () => {
      cancelled = true;
    };
  }, [definitions, kernel, model, objectRow]);
  if (!objectRow || !definitions.length) return null;
  const count = selectionCount ?? selection.length;
  return (
    <div className="flex flex-col gap-single text-xs">
      <span className="text-foreground text-sm font-semibold">Properties</span>
      <span className="text-muted-foreground text-2xs">
        object · <code className="text-foreground">{objectRow.id}</code>
        {count > 1 ? ` · ${count} selected` : ""}
      </span>
      {definitions.map((defn) => (
        <div key={defn.id} className="flex flex-col gap-half">
          <span>{defn.label}</span>
          <pre className="text-muted-foreground m-0 overflow-auto text-2xs">{JSON.stringify(values[defn.id] ?? {}, null, 2)}</pre>
        </div>
      ))}
    </div>
  );
}
// #endregion 🪩️Repl
