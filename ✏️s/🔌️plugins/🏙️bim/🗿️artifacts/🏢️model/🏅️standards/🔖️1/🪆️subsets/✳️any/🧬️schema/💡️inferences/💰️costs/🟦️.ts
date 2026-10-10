import type {CostBasis, CostItem, TypeCostLink} from "../../📸️snapshot/💰️costs/🟦️.ts";
import type {ElementQuantity} from "../🧮️quantities/🟦️.ts";

/** 📏️ The quantity fields consumed by costing; all information comes from the existing quantity inference. */
export type CostQuantity = Pick<ElementQuantity, "storey" | "type_id"> & Partial<Pick<ElementQuantity, "count" | "length" | "gross_area" | "net_area" | "surface_area" | "gross_side_area" | "net_side_area" | "gross_volume" | "net_volume" | "mass">>;

/** 🧮️ A derived cost line with enough authored and inferred inputs to explain its amount. */
export interface CostLine {
  element: string;
  link: string;
  item: string;
  type_id: string;
  storey: string;
  currency: string;
  quantity: number;
  unit_cost: number;
  factor: number;
  precision: number;
  total: number;
}

/** 🚫️ A failed cost input; consumers localize the machine code. */
export interface CostDiagnostic {
  code: "cost.item-invalid" | "cost.link-invalid" | "cost.item-missing" | "cost.quantity-invalid" | "cost.overflow";
  link: string;
  item: string;
  element: string;
}

/** 💰️ Derived amounts grouped by scope and currency, without currency conversion or authored totals. */
export interface ModelCosts {
  lines: CostLine[];
  elements: Record<string, Record<string, number>>;
  types: Record<string, Record<string, number>>;
  storeys: Record<string, Record<string, number>>;
  items: Record<string, Record<string, number>>;
  project: Record<string, number>;
  diagnostics: CostDiagnostic[];
}

type Decimal = {coefficient: bigint; exponent: number};
type Totals = Map<string, Map<string, Decimal>>;

const fields: Record<CostBasis, keyof CostQuantity> = {Count: "count", Length: "length", GrossArea: "gross_area", NetArea: "net_area", SurfaceArea: "surface_area", GrossSideArea: "gross_side_area", NetSideArea: "net_side_area", GrossVolume: "gross_volume", NetVolume: "net_volume", Mass: "mass"};
const power = (exponent: number): bigint => 10n ** BigInt(exponent);
const encoder = new TextEncoder();
const compare = (left: string, right: string): number => {
  const a = encoder.encode(left), b = encoder.encode(right);
  for (let i = 0; i < Math.min(a.length, b.length); i++) if (a[i] !== b[i]) return a[i]! - b[i]!;
  return a.length - b.length;
};
const decimal = (value: number): Decimal => {
  const [mantissa, exponent = "0"] = value.toString().split("e");
  const [whole, fraction = ""] = mantissa!.split(".");
  return {coefficient: BigInt(whole! + fraction), exponent: Number(exponent) - fraction.length};
};
const number = (value: Decimal): number => Number(value.coefficient.toString() + "e" + value.exponent);
const plus = (left: Decimal, right: Decimal): Decimal => {
  const exponent = Math.min(left.exponent, right.exponent);
  return {coefficient: left.coefficient * power(left.exponent - exponent) + right.coefficient * power(right.exponent - exponent), exponent};
};

/** 🔟️ Rounds the exact product of canonical decimal inputs using the authored nonnegative half-up policy. */
function product(terms: number[], precision: number): Decimal {
  let coefficient = 1n;
  let exponent = 0;
  for (const term of terms) {
    const value = decimal(term);
    coefficient *= value.coefficient;
    exponent += value.exponent;
  }
  const shift = exponent + precision;
  if (shift >= 0) return {coefficient: coefficient * power(shift), exponent: -precision};
  const divisor = power(-shift);
  return {coefficient: coefficient / divisor + (coefficient % divisor * 2n >= divisor ? 1n : 0n), exponent: -precision};
}

const nonnegative = (value: number): boolean => Number.isFinite(value) && value >= 0;
const validItem = (item: CostItem): boolean => item.name.trim().length > 0 && item.currency.trim().length > 0 && Object.hasOwn(fields, item.basis) && nonnegative(item.unit_cost) && Number.isInteger(item.precision) && item.precision >= 0 && item.precision <= 9;
const present = (totals: Totals): Record<string, Record<string, number>> => Object.fromEntries([...totals].map(([id, currencies]) => [id, Object.fromEntries([...currencies].map(([currency, value]) => [currency, number(value)]))]));

/** 💡️ Costs existing inferred quantity rows using authored items and links; no model or inference state is held. */
export function inferCosts(items: Readonly<Record<string, CostItem>>, links: Readonly<Record<string, TypeCostLink>>, elements: Readonly<Record<string, CostQuantity>>): ModelCosts {
  const result: ModelCosts = {lines: [], elements: {}, types: {}, storeys: {}, items: {}, project: {}, diagnostics: []};
  const scopes: Totals[] = Array.from({length: 5}, () => new Map());
  const byType = new Map<string, [string, TypeCostLink, CostItem][]>();
  for (const [id, link] of Object.entries(links).sort(([a], [b]) => compare(a, b))) {
    const item = Object.hasOwn(items, link.item) ? items[link.item] : undefined;
    const code = !link.type_id.trim() || !nonnegative(link.factor) ? "cost.link-invalid" : !item ? "cost.item-missing" : !validItem(item) ? "cost.item-invalid" : undefined;
    if (code) {result.diagnostics.push({code, link: id, item: link.item, element: ""}); continue;}
    const rows = byType.get(link.type_id) ?? [];
    rows.push([id, link, item!]);
    byType.set(link.type_id, rows);
  }
  for (const [element, measured] of Object.entries(elements).sort(([a], [b]) => compare(a, b))) {
    for (const [id, link, item] of byType.get(measured.type_id) ?? []) {
      const quantity = measured[fields[item.basis]] ?? 0;
      if (typeof quantity !== "number" || !nonnegative(quantity)) {result.diagnostics.push({code: "cost.quantity-invalid", link: id, item: link.item, element}); continue;}
      const amount = product([quantity, item.unit_cost, link.factor], item.precision);
      const keys = [element, measured.type_id, measured.storey, link.item, ""];
      const updates = scopes.map((scope, index) => plus(scope.get(keys[index]!)?.get(item.currency) ?? {coefficient: 0n, exponent: 0}, amount));
      if (!Number.isFinite(number(amount)) || updates.some(value => !Number.isFinite(number(value)))) {result.diagnostics.push({code: "cost.overflow", link: id, item: link.item, element}); continue;}
      scopes.forEach((scope, index) => {const currencies = scope.get(keys[index]!) ?? new Map(); currencies.set(item.currency, updates[index]!); scope.set(keys[index]!, currencies);});
      result.lines.push({element, link: id, item: link.item, type_id: measured.type_id, storey: measured.storey, currency: item.currency, quantity, unit_cost: item.unit_cost, factor: link.factor, precision: item.precision, total: number(amount)});
    }
  }
  [result.elements, result.types, result.storeys, result.items] = scopes.slice(0, 4).map(present) as [ModelCosts["elements"], ModelCosts["types"], ModelCosts["storeys"], ModelCosts["items"]];
  result.project = present(scopes[4]!)[""] ?? {};
  return result;
}
