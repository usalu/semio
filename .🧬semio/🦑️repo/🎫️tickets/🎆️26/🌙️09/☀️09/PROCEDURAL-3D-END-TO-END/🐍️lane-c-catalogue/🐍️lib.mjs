export const T = (en, de) => ({ en, de });
const PI = Math.PI;
export const RAD = PI / 180;

export const dict = {};
export const def = (name, spec) => { dict[name] = spec; };

const PORT_KEYS = ["name", "label", "description", "type", "shapeKinds", "selection", "list", "minItems", "maxItems", "default", "min", "exclusiveMin", "max", "step", "unit", "options", "optional"];
const order = (object, keys) => Object.fromEntries(keys.filter((key) => object[key] !== undefined).map((key) => [key, object[key]]));

const stepOf = (port) => {
  if (port.step !== undefined) return port.step;
  if (port.type === "integer") return 1;
  if (port.type === "angle") return RAD;
  if (port.type === "number" || port.type === "length") return 0.1;
  return undefined;
};

export const I = (name, over = {}) => {
  const spec = { ...(dict[name] ?? (() => { throw new Error(`unknown shared port ${name}`); })()), ...over };
  const port = { name: over.as ?? name, ...spec };
  delete port.as;
  delete port.was;
  if (["number", "integer", "angle", "length"].includes(port.type)) port.step = stepOf(port);
  if (port.type === "angle" && port.unit === undefined) port.unit = "rad";
  return order(port, PORT_KEYS);
};

export const O = (name, type, label, description, extra = {}) => order({ name, label, description, type, ...extra }, PORT_KEYS);

export const P = (name, type, label, description, extra = {}) => {
  const port = { name, label, description, type, ...extra };
  if (["number", "integer", "angle", "length"].includes(port.type)) port.step = stepOf(port);
  if (port.type === "angle" && port.unit === undefined) port.unit = "rad";
  return order(port, PORT_KEYS);
};

export const pick = (component, port) => ({ component, port });
export const gumball = (motion, port, extra = {}) => ({ motion, port, ...extra });

export const vs = (emoji) => (emoji.endsWith("\uFE0F") ? emoji : `${emoji}\uFE0F`);

export const KIND_KEYS = ["id", "category", "emoji", "label", "description", "inputs", "outputs", "quality", "interaction", "preview"];

export const K = (category, spec) => {
  const interaction = spec.interaction && ((spec.interaction.pick?.length ?? 0) + (spec.interaction.gumball?.length ?? 0) > 0) ? { ...(spec.interaction.pick?.length ? { pick: spec.interaction.pick } : {}), ...(spec.interaction.gumball?.length ? { gumball: spec.interaction.gumball } : {}) } : undefined;
  const kind = { id: spec.id, category, emoji: vs(spec.emoji), label: spec.label, description: spec.description, inputs: spec.in ?? [], outputs: spec.out ?? [], quality: spec.quality, interaction, preview: spec.preview ?? false };
  return { kind: order(kind, KIND_KEYS), was: spec.was, ren: spec.ren ?? {}, note: spec.note ?? "" };
};

export const sel = (component, source, extra = {}) => ({ selection: { component, source, multiple: extra.multiple ?? true, ...(extra.modeFrom ? { modeFrom: extra.modeFrom } : {}) } });

export const lenOpts = { min: 0, exclusiveMin: true };
export const SHAPE_KINDS = ["solid", "shell", "face", "wire", "edge", "curve", "surface", "vertex", "compound"];
