// #region 🧲Header
// Drives PLACEHOLDER_* ids in JSON; lists entities from the live snapshot.
// #endregion

import * as React from "react";

import type { KitStoreHandle } from "./semioWasm";

export const EntityPicker: React.FC<{
  handle: KitStoreHandle | null;
  onApplyPlaceholders: (s: string) => string;
  jsonForPlaceholders: string;
  onJsonChange: (s: string) => void;
}> = ({ handle, onApplyPlaceholders, jsonForPlaceholders, onJsonChange }) => {
  const snap = handle?.snapshot() as
    | {
        types?: { id: string; name?: string }[];
        designs?: { id: string; name?: string }[];
        files?: { id: string }[];
        folders?: { id: string }[];
        authors?: { id: string }[];
      }
    | undefined;

  const [ti, setTi] = React.useState(0);
  const [di, setDi] = React.useState(0);
  const [fi, setFi] = React.useState(0);
  const [foi, setFoi] = React.useState(0);
  const [ai, setAi] = React.useState(0);

  const t = snap?.types ?? [];
  const d = snap?.designs ?? [];
  const f = snap?.files ?? [];
  const fo = snap?.folders ?? [];
  const a = snap?.authors ?? [];

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 overflow-auto border-b border-zinc-200 p-2 text-xs dark:border-zinc-800">
      <div className="text-muted-foreground font-medium">Entity ids (for JSON placeholders)</div>
      {!snap ? <div className="text-muted-foreground">(no kit)</div> : null}
      <div className="grid max-w-full grid-cols-1 gap-1 text-[10px]">
        <L sel={ti} set={setTi} label="Type" options={t.map((x) => ({ id: x.id, n: x.name ?? x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_TYPE_ID" />
        <L sel={di} set={setDi} label="Design" options={d.map((x) => ({ id: x.id, n: x.name ?? x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_DESIGN_ID" />
        <L sel={fi} set={setFi} label="File" options={f.map((x) => ({ id: x.id, n: x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_FILE_ID" />
        <L sel={foi} set={setFoi} label="Folder" options={fo.map((x) => ({ id: x.id, n: x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_FOLDER_ID" />
        <L sel={ai} set={setAi} label="Author" options={a.map((x) => ({ id: x.id, n: x.id }))} onPick={onJsonChange} json={jsonForPlaceholders} ph="PLACEHOLDER_AUTHOR_ID" />
      </div>
      <button
        type="button"
        className="mt-1 w-full rounded border border-cyan-600 px-1 py-0.5 text-[10px] text-cyan-800 dark:text-cyan-200"
        onClick={() => onJsonChange(onApplyPlaceholders(jsonForPlaceholders))}
      >
        Replace all PLACEHOLDER_* in command JSON
      </button>
    </div>
  );
};

const L: React.FC<{
  label: string;
  options: { id: string; n: string }[];
  sel: number;
  set: (n: number) => void;
  json: string;
  onPick: (s: string) => void;
  ph: string;
}> = ({ label, options, sel, set, onPick, json, ph }) => (
  <label className="text-muted-foreground flex flex-wrap items-center gap-1">
    {label}
    <select
      className="bg-background max-w-full flex-1 rounded border border-zinc-300 px-1 py-0.5 font-mono dark:border-zinc-600"
      value={String(sel)}
      onChange={(e) => {
        const n = Number(e.target.value);
        set(n);
        const id = options[n]?.id;
        if (id) onPick(json.split(ph).join(id));
      }}
    >
      {options.length === 0 ? (
        <option value={0}>(empty)</option>
      ) : (
        options.map((o, i) => (
          <option key={o.id} value={i}>
            {o.n}
          </option>
        ))
      )}
    </select>
  </label>
);

/** Replace all known PLACEHOLDER_* in one pass. */
export function applyEntityPlaceholders(
  s: string,
  ctx: { typeId: string; designId: string; fileId: string; folderId: string; authorId: string; pieceId: string; connectionId: string },
): string {
  return s
    .split("PLACEHOLDER_TYPE_ID")
    .join(ctx.typeId)
    .split("PLACEHOLDER_DESIGN_ID")
    .join(ctx.designId)
    .split("PLACEHOLDER_FILE_ID")
    .join(ctx.fileId)
    .split("PLACEHOLDER_FOLDER_ID")
    .join(ctx.folderId)
    .split("PLACEHOLDER_AUTHOR_ID")
    .join(ctx.authorId)
    .split("PLACEHOLDER_PIECE_ID")
    .join(ctx.pieceId)
    .split("PLACEHOLDER_CONNECTION_ID")
    .join(ctx.connectionId);
}
