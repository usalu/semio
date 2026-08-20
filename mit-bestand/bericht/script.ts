#!/usr/bin/env bun
/** 🏚️ `@semio-tech/mit-bestand-bericht` router: `bun ./script.ts build|watch|latex`. */
import { existsSync, readFileSync, statSync, watch, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { basename, dirname, join, resolve } from "node:path";
import { forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY } from "d3-force";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/index.ts";
import { buildPrintDocument, fetchPrintFonts } from "../../print/script.ts";

const berichtRoot = import.meta.dir;

//#region Akteursnetz
const ACTOR_NETWORK_ROOT = join(berichtRoot, "forschungsbericht/anhang");
const ACTOR_NETWORK_LEDGER = join(ACTOR_NETWORK_ROOT, "akteursnetz-erweiterung-kandidaten.md");
const ACTOR_NETWORK_FIGURES = join(ACTOR_NETWORK_ROOT, "akteursnetz-figuren.tex");
const ACTOR_NETWORK_TABLES = join(ACTOR_NETWORK_ROOT, "akteursnetz-tabellen.tex");
const ACTOR_NETWORK_PROGRAMS = join(ACTOR_NETWORK_ROOT, "akteursnetz-programme.tex");
const ACTOR_NETWORK_INTRO = join(ACTOR_NETWORK_ROOT, "akteursnetz.tex");
const COUNTRIES = ["FR", "GB", "NL", "CH", "DE", "BE", "DK", "SE", "NO", "FI", "AT"] as const;
const COUNTRY_NAMES: Record<string, string> = {
  AT: "Österreich", BE: "Belgien", CH: "Schweiz", DE: "Deutschland", DK: "Dänemark", FI: "Finnland",
  FR: "Frankreich", GB: "Vereinigtes Königreich", NL: "Niederlande", NO: "Norwegen", SE: "Schweden",
};

type ActorDecision = {
  key: string;
  subject: string;
  decision: "keep" | "prune" | "merge";
  reason: string;
  target?: string;
};

type ProjectDecision = {
  key: string;
  decision: "keep" | "prune";
  reason: string;
};

type RoleEvidence = {
  url: string;
  quote?: string;
  note?: string;
  supportsRoles: string[];
  accessedAt?: string;
  reviewedAt?: string;
};

type ProjectEvidence = {
  url: string;
  quote?: string;
  note?: string;
  reviewedAt?: string;
};

type ActorNode = {
  key: string;
  country: string;
  kind: "actor" | "project";
  entityType: string;
  name: string;
  grade: "kern" | "bezug";
  roles: string[];
  roleGroups: string[];
  relevance: string;
  sourceUrl: string;
  roleEvidence?: RoleEvidence[];
  assetPath?: string;
  state?: "default" | "hypo" | "attested" | "focal";
  legacyId?: string;
  projectRef?: string;
  actualReuseVerified?: boolean;
  projectStatus?: "completed" | "under-construction" | "not-documented";
  reuseState?: "installed" | "salvaged-for-reuse" | "physical-reuse-verified";
  reuseObjects?: string[];
  reuseProcess?: string;
  projectEvidence?: ProjectEvidence;
  x?: number;
  y?: number;
};

type ActorEdge = {
  key: string;
  source: string;
  target: string;
  type: string;
  description: string;
  evidenceUrl?: string;
  evidenceQuote?: string;
  reviewBasis?: "retained-verified-baseline";
  kind?: "normal" | "muted";
};

type ActorProgram = {
  key: string;
  country: string;
  name: string;
  roles: string[];
  relevance: string;
  sourceUrl?: string;
  evidenceUrl?: string;
  evidenceQuote?: string;
  reviewedAt?: string;
};

type ActorNetworkData = {
  schemaVersion: 1;
  approved: boolean;
  baseline: { nodes: number; edges: number; projects: number; programs: number };
  baselineCleanup: Array<{ key: string; decision: "prune" | "program"; reason: string }>;
  projectDecisions: ProjectDecision[];
  decisions: ActorDecision[];
  nodes: ActorNode[];
  edges: ActorEdge[];
  programs: ActorProgram[];
};

type LayoutNode = ActorNode & {
  id: string;
  anchorX: number;
  anchorY: number;
  radius: number;
  vx?: number;
  vy?: number;
};

type LayoutLink = {
  key: string;
  source: string | LayoutNode;
  target: string | LayoutNode;
  kind?: "normal" | "muted";
};

type LayoutMetrics = {
  overlaps: number;
  duplicatePositions: number;
  outOfBounds: number;
  edgeNodeIntersections: number;
  edgeCrossings: number;
  legacyDisplacement: number;
  totalEdgeLength: number;
};

type LayoutPanel = {
  country: string;
  part?: { index: number; total: number };
  width: number;
  height: number;
  nodes: LayoutNode[];
  edges: ActorEdge[];
  metrics: LayoutMetrics;
};

function actorNetworkData(): ActorNetworkData {
  const markdown = readFileSync(ACTOR_NETWORK_LEDGER, "utf8");
  const match = markdown.match(/<!-- FINAL-DATA:START -->\s*```json\s*([\s\S]*?)```\s*<!-- FINAL-DATA:END -->/);
  if (!match) throw new Error(`missing FINAL-DATA block: ${ACTOR_NETWORK_LEDGER}`);
  return JSON.parse(match[1]) as ActorNetworkData;
}

function normalizedName(value: string): string {
  return value.normalize("NFKD").replace(/[\u0300-\u036f]/g, "").replace(/ß/g, "ss").replace(/æ/g, "ae")
    .replace(/ø/g, "o").toLowerCase().replace(/[^a-z0-9]+/g, "");
}

function displayIds(nodes: ActorNode[]): Map<string, string> {
  const result = new Map<string, string>();
  for (const country of COUNTRIES) {
    const countryNodes = nodes.filter((node) => node.country === country);
    for (const type of [...new Set(countryNodes.map((node) => node.entityType))].sort()) {
      countryNodes.filter((node) => node.entityType === type)
        .sort((a, b) => normalizedName(a.name).localeCompare(normalizedName(b.name)) || a.key.localeCompare(b.key))
        .forEach((node, index) => result.set(node.key, `${type}${type === "P" ? index + 1 : String(index + 1).padStart(2, "0")}`));
    }
  }
  return result;
}

function tex(value: string): string {
  return value.replace(/\\/g, "\\textbackslash{}")
    .replace(/([&%$#_{}])/g, "\\$1").replace(/~/g, "\\textasciitilde{}").replace(/\^/g, "\\textasciicircum{}");
}

function validateActorNetwork(data = actorNetworkData()): void {
  const errors: string[] = [];
  if (data.schemaVersion !== 1) errors.push("schemaVersion must be 1");
  if (!data.approved) errors.push("FINAL-DATA is not approved");
  if (data.nodes.length !== 798) errors.push(`expected 798 nodes, got ${data.nodes.length}`);
  if (data.programs.length !== 14) errors.push(`expected 14 programs, got ${data.programs.length}`);
  if (data.baseline.nodes !== 618 || data.baseline.edges !== 264 || data.baseline.projects !== 78 || data.baseline.programs !== 9)
    errors.push("baseline fingerprint differs from 618/264/78/9");
  if (!data.baselineCleanup?.length) errors.push("missing baseline cleanup provenance");
  for (const item of data.baselineCleanup ?? []) {
    if (!item.key.trim() || !item.reason.trim()) errors.push("incomplete baseline cleanup decision");
  }
  if (data.projectDecisions?.length !== 71) errors.push(`expected 71 project decisions, got ${data.projectDecisions?.length ?? 0}`);
  const projectDecisionKeys = new Set<string>();
  for (const item of data.projectDecisions ?? []) {
    if (projectDecisionKeys.has(item.key)) errors.push(`duplicate project decision ${item.key}`);
    projectDecisionKeys.add(item.key);
    if (!item.reason.trim()) errors.push(`missing project decision reason ${item.key}`);
  }
  const decisionKeys = new Set<string>();
  for (const decision of data.decisions) {
    if (decisionKeys.has(decision.key)) errors.push(`duplicate decision ${decision.key}`);
    decisionKeys.add(decision.key);
    if (!decision.reason.trim()) errors.push(`missing decision reason ${decision.key}`);
  }
  if (data.decisions.length !== 248) errors.push(`expected 248 decisions, got ${data.decisions.length}`);
  const nodes = new Map<string, ActorNode>();
  const actorNames = new Set<string>();
  const forbidden = /(?:textellipsis|…|Direkt belegt im baulichen Reuse-Kontext|Rolle noch offen|Unzureichende Informationen|Reuse-Bezug belegt|Rolle unklar|Keine direkte Reuse-Rolle belegt|kein direkter Beitrag zur Wiederverwendung|konkrete Funktion bleibt[^.]*unklar|Reuse-Umfang[^.]*unklar)/i;
  for (const node of data.nodes) {
    if (nodes.has(node.key)) errors.push(`duplicate node key ${node.key}`);
    nodes.set(node.key, node);
    if (!COUNTRIES.includes(node.country as typeof COUNTRIES[number])) errors.push(`invalid country ${node.key}`);
    if (!node.name.trim() || forbidden.test(node.name)) errors.push(`invalid name ${node.key}: ${node.name}`);
    if (!node.relevance.trim() || node.relevance.length > 90 || forbidden.test(node.relevance)) errors.push(`invalid relevance ${node.key}: ${node.relevance}`);
    if (!node.sourceUrl.startsWith("http")) errors.push(`missing source URL ${node.key}`);
    if (node.kind === "project") {
      if (node.entityType !== "P" || node.roles.length !== 1 || node.roles[0] !== "Referenzprojekt") errors.push(`invalid project role ${node.key}`);
      if (!node.actualReuseVerified) errors.push(`project lacks actual reuse proof ${node.key}`);
      if (!node.projectStatus || !node.reuseState || !node.reuseObjects?.length || !node.reuseProcess?.trim()) errors.push(`incomplete project evidence ${node.key}`);
      if (!node.projectEvidence?.url.startsWith("http") || !(node.projectEvidence.quote?.trim() || node.projectEvidence.note?.trim())) errors.push(`missing project source excerpt ${node.key}`);
    } else if (node.roles.length < 1 || node.roles.length > 3 || node.roleGroups.length < 1 || node.roleGroups.length > 3) {
      errors.push(`invalid actor roles ${node.key}`);
    } else {
      const actorName = normalizedName(node.name);
      if (actorNames.has(actorName)) errors.push(`duplicate actor name ${node.name}`);
      actorNames.add(actorName);
      for (const role of node.roles) {
        const evidence = node.roleEvidence?.find((item) => item.supportsRoles.includes(role));
        if (!evidence?.url.startsWith("http") || !(evidence.quote?.trim() || evidence.note?.trim())) errors.push(`missing role evidence ${node.key}: ${role}`);
      }
    }
    if (node.assetPath && !existsSync(join(berichtRoot, "forschungsbericht", node.assetPath))) errors.push(`missing asset ${node.key}: ${node.assetPath}`);
    if (/Claus Asam/i.test(node.name)) errors.push("Claus Asam must not render");
  }
  const edgeKeys = new Set<string>();
  const edgeTriples = new Set<string>();
  for (const edge of data.edges) {
    if (edgeKeys.has(edge.key)) errors.push(`duplicate edge key ${edge.key}`);
    edgeKeys.add(edge.key);
    if (!nodes.has(edge.source) || !nodes.has(edge.target)) errors.push(`dangling edge ${edge.key}`);
    if (edge.source === edge.target) errors.push(`self edge ${edge.key}`);
    const endpoints = [edge.source, edge.target].sort();
    const triple = `${endpoints[0]}|${endpoints[1]}|${edge.type}`;
    if (edgeTriples.has(triple)) errors.push(`duplicate edge ${triple}`);
    edgeTriples.add(triple);
    if (!edge.reviewBasis && (!edge.evidenceUrl?.startsWith("http") || !edge.evidenceQuote?.trim())) errors.push(`missing edge evidence ${edge.key}`);
    if (/Rolle noch offen|genaue Rolle noch offen/i.test(edge.description)) errors.push(`open edge ${edge.key}`);
  }
  for (const decision of data.decisions.filter((item) => item.decision === "merge")) {
    if (!decision.target || !nodes.has(decision.target)) errors.push(`invalid merge target ${decision.key}`);
  }
  for (const decision of data.decisions) {
    const proposal = decision.key.match(/^proposal:(proj:\d+):([AB]):(\d+)$/);
    const expectedNode = proposal?.[2] === "B" ? `candidate:${proposal[1]}:B:${proposal[3]}` : undefined;
    const expectedEdge = proposal?.[2] === "A" ? `candidate-edge:${decision.key}` : undefined;
    if (decision.decision === "keep" && expectedNode && !nodes.has(expectedNode)) errors.push(`kept candidate lacks node ${decision.key}`);
    if (decision.decision === "keep" && expectedEdge && !edgeKeys.has(expectedEdge)) errors.push(`kept candidate lacks edge ${decision.key}`);
    if (decision.decision !== "keep" && expectedNode && nodes.has(expectedNode)) errors.push(`non-kept candidate rendered ${decision.key}`);
    if (decision.decision !== "keep" && expectedEdge && edgeKeys.has(expectedEdge)) errors.push(`non-kept candidate edge rendered ${decision.key}`);
  }
  for (const decision of data.projectDecisions ?? []) {
    if ((decision.decision === "keep") !== nodes.has(decision.key)) errors.push(`project decision/render mismatch ${decision.key}`);
  }
  for (const node of data.nodes.filter((item) => item.key.startsWith("proj:"))) {
    if (!projectDecisionKeys.has(node.key)) errors.push(`extension project lacks decision ${node.key}`);
  }
  const programKeys = new Set<string>();
  const nodeNames = new Set(data.nodes.map((node) => `${node.country}|${normalizedName(node.name)}`));
  for (const program of data.programs) {
    if (programKeys.has(program.key)) errors.push(`duplicate program key ${program.key}`);
    programKeys.add(program.key);
    if (!COUNTRIES.includes(program.country as typeof COUNTRIES[number])) errors.push(`invalid program country ${program.key}`);
    if (!program.name.trim() || !program.roles.length || !program.relevance.trim() || program.relevance.length > 90)
      errors.push(`invalid program ${program.key}`);
    if (!program.sourceUrl?.startsWith("http")) errors.push(`missing program source ${program.key}`);
    if (!program.evidenceUrl?.startsWith("http") || !program.evidenceQuote?.trim() || !program.reviewedAt?.trim()) errors.push(`missing program evidence ${program.key}`);
    if (nodeNames.has(`${program.country}|${normalizedName(program.name)}`)) errors.push(`program also rendered as actor ${program.key}`);
  }
  const display = displayIds(data.nodes);
  const ids = data.nodes.map((node) => `${node.country}:${display.get(node.key)}`);
  if (new Set(ids).size !== ids.length) errors.push("duplicate visible IDs within a country");
  if (errors.length) throw new Error(`Akteursnetz validation failed (${errors.length}):\n${errors.slice(0, 80).join("\n")}`);
  console.log(`[DEBUG] Akteursnetz valid: ${data.nodes.length} nodes, ${data.edges.length} edges, ${data.programs.length} programs`);
}

const GRAPH_WIDTH = 181;
const GRAPH_HEIGHTS = [45, 60, 75, 90] as const;
const GRAPH_ACTOR_RADIUS = 2.275;
const GRAPH_PROJECT_RADIUS = 2.575;
const GRAPH_NODE_PADDING = 0.4;
const GRAPH_OUTPUT_SAFETY = 0.02;
const GRAPH_LAYOUT_ATTEMPTS = 8;

function graphHeight(count: number): number {
  if (count <= 60) return 45;
  if (count <= 90) return 60;
  if (count <= 110) return 75;
  return 90;
}

function stableHash(value: string, salt = 0): number {
  let hash = 2166136261 ^ salt;
  for (const char of value) hash = Math.imul(hash ^ char.charCodeAt(0), 16777619);
  return hash >>> 0;
}

function seededRandom(value: string, salt: number): () => number {
  let state = stableHash(value, salt) || 0x6d2b79f5;
  return () => {
    state += 0x6d2b79f5;
    let next = state;
    next = Math.imul(next ^ (next >>> 15), next | 1);
    next ^= next + Math.imul(next ^ (next >>> 7), next | 61);
    return ((next ^ (next >>> 14)) >>> 0) / 0x100000000;
  };
}

function halton(index: number, base: number): number {
  let fraction = 1;
  let result = 0;
  while (index > 0) {
    fraction /= base;
    result += fraction * (index % base);
    index = Math.floor(index / base);
  }
  return result;
}

function graphNodeRadius(node: ActorNode): number {
  return node.kind === "project" || node.state === "focal" ? GRAPH_PROJECT_RADIUS : GRAPH_ACTOR_RADIUS;
}

function layoutBoundaryForce(width: number, height: number) {
  let nodes: LayoutNode[] = [];
  const force = (alpha: number) => {
    for (const node of nodes) {
      const margin = node.radius + GRAPH_NODE_PADDING;
      const x = node.x ?? width / 2;
      const y = node.y ?? height / 2;
      if (x < margin) node.vx = (node.vx ?? 0) + (margin - x) * 0.9 * alpha;
      if (x > width - margin) node.vx = (node.vx ?? 0) - (x - width + margin) * 0.9 * alpha;
      if (y < margin) node.vy = (node.vy ?? 0) + (margin - y) * 0.9 * alpha;
      if (y > height - margin) node.vy = (node.vy ?? 0) - (y - height + margin) * 0.9 * alpha;
    }
  };
  force.initialize = (next: LayoutNode[]) => { nodes = next; };
  return force;
}

function clampLayoutNode(node: LayoutNode, width: number, height: number): void {
  const margin = node.radius + GRAPH_NODE_PADDING + GRAPH_OUTPUT_SAFETY;
  node.x = Math.max(margin, Math.min(width - margin, node.x ?? width / 2));
  node.y = Math.max(margin, Math.min(height - margin, node.y ?? height / 2));
}

function relaxNodeOverlaps(nodes: LayoutNode[], width: number, height: number, sweeps = 320): void {
  for (let sweep = 0; sweep < sweeps; sweep++) {
    let maximumOverlap = 0;
    for (const node of nodes) clampLayoutNode(node, width, height);
    for (let left = 0; left < nodes.length; left++) {
      for (let right = left + 1; right < nodes.length; right++) {
        const a = nodes[left];
        const b = nodes[right];
        const minimum = a.radius + b.radius + GRAPH_NODE_PADDING * 2 + 0.035;
        let dx = (b.x ?? 0) - (a.x ?? 0);
        let dy = (b.y ?? 0) - (a.y ?? 0);
        let distance = Math.hypot(dx, dy);
        if (distance >= minimum) continue;
        if (distance < 1e-9) {
          const angle = (stableHash(`${a.key}|${b.key}`) / 0xffffffff) * Math.PI * 2;
          dx = Math.cos(angle);
          dy = Math.sin(angle);
          distance = 1;
        }
        const overlap = minimum - distance;
        maximumOverlap = Math.max(maximumOverlap, overlap);
        const mobilityA = a.legacyId ? 0.35 : 1;
        const mobilityB = b.legacyId ? 0.35 : 1;
        const mobility = mobilityA + mobilityB;
        const ux = dx / distance;
        const uy = dy / distance;
        a.x = (a.x ?? 0) - ux * overlap * mobilityA / mobility;
        a.y = (a.y ?? 0) - uy * overlap * mobilityA / mobility;
        b.x = (b.x ?? 0) + ux * overlap * mobilityB / mobility;
        b.y = (b.y ?? 0) + uy * overlap * mobilityB / mobility;
      }
    }
    if (maximumOverlap < 0.002) break;
  }
  for (const node of nodes) clampLayoutNode(node, width, height);
}

function pointSegmentDistance(px: number, py: number, ax: number, ay: number, bx: number, by: number): number {
  const dx = bx - ax;
  const dy = by - ay;
  const lengthSquared = dx * dx + dy * dy;
  if (lengthSquared === 0) return Math.hypot(px - ax, py - ay);
  const t = Math.max(0, Math.min(1, ((px - ax) * dx + (py - ay) * dy) / lengthSquared));
  return Math.hypot(px - (ax + t * dx), py - (ay + t * dy));
}

function relaxEdgeNodeIntersections(nodes: LayoutNode[], edges: ActorEdge[], width: number, height: number): void {
  const byKey = new Map(nodes.map((node) => [node.key, node]));
  for (let pass = 0; pass < 90; pass++) {
    let changes = 0;
    for (const edge of edges) {
      const source = byKey.get(edge.source)!;
      const target = byKey.get(edge.target)!;
      const ax = source.x ?? 0;
      const ay = source.y ?? 0;
      const bx = target.x ?? 0;
      const by = target.y ?? 0;
      const edgeX = bx - ax;
      const edgeY = by - ay;
      const edgeLengthSquared = edgeX * edgeX + edgeY * edgeY;
      if (edgeLengthSquared === 0) continue;
      for (const node of nodes) {
        if (node.key === edge.source || node.key === edge.target) continue;
        const px = node.x ?? 0;
        const py = node.y ?? 0;
        const t = Math.max(0, Math.min(1, ((px - ax) * edgeX + (py - ay) * edgeY) / edgeLengthSquared));
        if (t <= 0.03 || t >= 0.97) continue;
        const nearestX = ax + t * edgeX;
        const nearestY = ay + t * edgeY;
        let dx = px - nearestX;
        let dy = py - nearestY;
        let distance = Math.hypot(dx, dy);
        const clearance = node.radius + GRAPH_NODE_PADDING + 0.08;
        if (distance >= clearance) continue;
        if (distance < 1e-9) {
          const side = stableHash(`${edge.key}|${node.key}`) % 2 === 0 ? 1 : -1;
          const edgeLength = Math.sqrt(edgeLengthSquared);
          dx = -edgeY / edgeLength * side;
          dy = edgeX / edgeLength * side;
          distance = 1;
        }
        const push = clearance - distance;
        node.x = px + dx / distance * push;
        node.y = py + dy / distance * push;
        clampLayoutNode(node, width, height);
        changes++;
      }
    }
    relaxNodeOverlaps(nodes, width, height, 8);
    if (changes === 0) break;
  }
}

function segmentsCross(a: LayoutNode, b: LayoutNode, c: LayoutNode, d: LayoutNode): boolean {
  const orientation = (p: LayoutNode, q: LayoutNode, r: LayoutNode) =>
    ((q.x ?? 0) - (p.x ?? 0)) * ((r.y ?? 0) - (p.y ?? 0)) - ((q.y ?? 0) - (p.y ?? 0)) * ((r.x ?? 0) - (p.x ?? 0));
  const abC = orientation(a, b, c);
  const abD = orientation(a, b, d);
  const cdA = orientation(c, d, a);
  const cdB = orientation(c, d, b);
  return abC * abD < -1e-8 && cdA * cdB < -1e-8;
}

function layoutMetrics(nodes: LayoutNode[], edges: ActorEdge[], width: number, height: number): LayoutMetrics {
  const byKey = new Map(nodes.map((node) => [node.key, node]));
  let overlaps = 0;
  let duplicatePositions = 0;
  let outOfBounds = 0;
  let edgeNodeIntersections = 0;
  let edgeCrossings = 0;
  let legacyDisplacement = 0;
  let totalEdgeLength = 0;
  for (const node of nodes) {
    const margin = node.radius + GRAPH_NODE_PADDING;
    if ((node.x ?? 0) < margin - 0.005 || (node.x ?? 0) > width - margin + 0.005 ||
      (node.y ?? 0) < margin - 0.005 || (node.y ?? 0) > height - margin + 0.005) outOfBounds++;
    if (node.legacyId) legacyDisplacement += Math.hypot((node.x ?? 0) - node.anchorX, (node.y ?? 0) - node.anchorY);
  }
  for (let left = 0; left < nodes.length; left++) {
    for (let right = left + 1; right < nodes.length; right++) {
      const distance = Math.hypot((nodes[left].x ?? 0) - (nodes[right].x ?? 0), (nodes[left].y ?? 0) - (nodes[right].y ?? 0));
      if (distance < 0.005) duplicatePositions++;
      if (distance < nodes[left].radius + nodes[right].radius + GRAPH_NODE_PADDING * 2 - 0.005) overlaps++;
    }
  }
  for (const edge of edges) {
    const source = byKey.get(edge.source)!;
    const target = byKey.get(edge.target)!;
    totalEdgeLength += Math.hypot((source.x ?? 0) - (target.x ?? 0), (source.y ?? 0) - (target.y ?? 0));
    for (const node of nodes) {
      if (node.key === edge.source || node.key === edge.target) continue;
      if (pointSegmentDistance(node.x ?? 0, node.y ?? 0, source.x ?? 0, source.y ?? 0, target.x ?? 0, target.y ?? 0) < node.radius + GRAPH_NODE_PADDING - 0.005)
        edgeNodeIntersections++;
    }
  }
  for (let left = 0; left < edges.length; left++) {
    for (let right = left + 1; right < edges.length; right++) {
      const a = edges[left];
      const b = edges[right];
      if (a.source === b.source || a.source === b.target || a.target === b.source || a.target === b.target) continue;
      if (segmentsCross(byKey.get(a.source)!, byKey.get(a.target)!, byKey.get(b.source)!, byKey.get(b.target)!)) edgeCrossings++;
    }
  }
  return { overlaps, duplicatePositions, outOfBounds, edgeNodeIntersections, edgeCrossings, legacyDisplacement, totalEdgeLength };
}

function compareLayoutMetrics(left: LayoutMetrics, right: LayoutMetrics): number {
  const leftValues = [left.overlaps, left.duplicatePositions, left.outOfBounds, left.edgeNodeIntersections,
    left.edgeCrossings, left.legacyDisplacement, left.totalEdgeLength];
  const rightValues = [right.overlaps, right.duplicatePositions, right.outOfBounds, right.edgeNodeIntersections,
    right.edgeCrossings, right.legacyDisplacement, right.totalEdgeLength];
  for (let index = 0; index < leftValues.length; index++) if (leftValues[index] !== rightValues[index]) return leftValues[index] - rightValues[index];
  return 0;
}

function layoutPasses(metrics: LayoutMetrics): boolean {
  return metrics.overlaps === 0 && metrics.duplicatePositions === 0 && metrics.outOfBounds === 0 && metrics.edgeNodeIntersections === 0;
}

function countryLayout(nodes: ActorNode[], edges: ActorEdge[], width: number, height: number, attempt: number): LayoutNode[] {
  const seedKey = nodes.map((node) => node.key).sort().join("|");
  const random = seededRandom(seedKey, attempt + Math.round(height * 100));
  const adjacency = new Map(nodes.map((node) => [node.key, [] as string[]]));
  for (const edge of edges) {
    adjacency.get(edge.source)?.push(edge.target);
    adjacency.get(edge.target)?.push(edge.source);
  }
  const anchors = new Map<string, { x: number; y: number }>();
  for (const node of nodes) if (node.x !== undefined && node.y !== undefined) anchors.set(node.key, {
    x: node.x,
    y: GRAPH_PROJECT_RADIUS + GRAPH_NODE_PADDING + ((node.y - 3) / 39) * (height - (GRAPH_PROJECT_RADIUS + GRAPH_NODE_PADDING) * 2),
  });
  const unresolved = nodes.filter((node) => !anchors.has(node.key)).sort((a, b) => a.key.localeCompare(b.key));
  for (const [index, node] of unresolved.entries()) {
    const references = [node.projectRef, ...(adjacency.get(node.key) ?? [])].filter((key): key is string => !!key)
      .map((key) => anchors.get(key)).filter((value): value is { x: number; y: number } => !!value);
    if (references.length) {
      const centerX = references.reduce((sum, item) => sum + item.x, 0) / references.length;
      const centerY = references.reduce((sum, item) => sum + item.y, 0) / references.length;
      const angle = (stableHash(node.key, 41) / 0xffffffff) * Math.PI * 2;
      anchors.set(node.key, { x: centerX + Math.cos(angle) * 9, y: centerY + Math.sin(angle) * 9 });
    } else {
      const sequence = index + 1 + attempt * unresolved.length;
      anchors.set(node.key, {
        x: 4 + halton(sequence, 2) * (width - 8),
        y: 4 + halton(sequence, 3) * (height - 8),
      });
    }
  }
  const layout = nodes.map((node) => {
    const anchor = anchors.get(node.key)!;
    const jitter = node.legacyId ? 0.2 : 1.8;
    return { ...node, id: node.key, anchorX: anchor.x, anchorY: anchor.y, radius: graphNodeRadius(node),
      x: anchor.x + (random() - 0.5) * jitter, y: anchor.y + (random() - 0.5) * jitter };
  });
  const links: LayoutLink[] = edges.map((edge) => ({ key: edge.key, source: edge.source, target: edge.target, kind: edge.kind }));
  const simulation = forceSimulation(layout).randomSource(random).alpha(1).alphaMin(0.001)
    .alphaDecay(1 - Math.pow(0.001, 1 / 600)).velocityDecay(0.38)
    .force("link", forceLink<LayoutNode, LayoutLink>(links).id((node) => node.id)
      .distance((link) => link.kind === "muted" ? 16 : 12).strength((link) => link.kind === "muted" ? 0.06 : 0.18).iterations(2))
    .force("charge", forceManyBody<LayoutNode>().strength(-18).distanceMax(42))
    .force("collide", forceCollide<LayoutNode>().radius((node) => node.radius + GRAPH_NODE_PADDING).strength(1).iterations(3))
    .force("x", forceX<LayoutNode>((node) => node.anchorX).strength((node) => node.legacyId ? 0.055 : 0.018))
    .force("y", forceY<LayoutNode>((node) => node.anchorY).strength((node) => node.legacyId ? 0.055 : 0.018))
    .force("bounds", layoutBoundaryForce(width, height)).stop();
  for (let index = 0; index < 600; index++) simulation.tick();
  relaxNodeOverlaps(layout, width, height);
  relaxEdgeNodeIntersections(layout, edges, width, height);
  relaxNodeOverlaps(layout, width, height);
  for (const node of layout) {
    node.x = Math.round((node.x ?? width / 2) * 100) / 100;
    node.y = Math.round((node.y ?? height / 2) * 100) / 100;
  }
  return layout;
}

function bestLayoutPanel(country: string, nodes: ActorNode[], edges: ActorEdge[], width: number, height: number,
  part?: { index: number; total: number }): LayoutPanel {
  let best: LayoutPanel | undefined;
  for (let attempt = 0; attempt < GRAPH_LAYOUT_ATTEMPTS; attempt++) {
    const layout = countryLayout(nodes, edges, width, height, attempt);
    const metrics = layoutMetrics(layout, edges, width, height);
    const candidate = { country, part, width, height, nodes: layout, edges, metrics };
    if (!best || compareLayoutMetrics(candidate.metrics, best.metrics) < 0) best = candidate;
  }
  return best!;
}

function connectedComponents(nodes: ActorNode[], edges: ActorEdge[]): ActorNode[][] {
  const byKey = new Map(nodes.map((node) => [node.key, node]));
  const adjacency = new Map(nodes.map((node) => [node.key, [] as string[]]));
  for (const edge of edges) {
    adjacency.get(edge.source)?.push(edge.target);
    adjacency.get(edge.target)?.push(edge.source);
  }
  const seen = new Set<string>();
  const result: ActorNode[][] = [];
  for (const start of [...nodes].sort((a, b) => a.key.localeCompare(b.key))) {
    if (seen.has(start.key)) continue;
    const queue = [start.key];
    const component: ActorNode[] = [];
    seen.add(start.key);
    while (queue.length) {
      const key = queue.shift()!;
      component.push(byKey.get(key)!);
      for (const next of adjacency.get(key) ?? []) if (!seen.has(next)) { seen.add(next); queue.push(next); }
    }
    result.push(component);
  }
  return result;
}

function countryPanels(country: string, nodes: ActorNode[], edges: ActorEdge[]): LayoutPanel[] {
  const minimumHeight = graphHeight(nodes.length);
  for (const height of GRAPH_HEIGHTS.filter((candidate) => candidate >= minimumHeight)) {
    const panel = bestLayoutPanel(country, nodes, edges, GRAPH_WIDTH, height);
    if (layoutPasses(panel.metrics)) return [panel];
  }
  const groups: ActorNode[][] = [[], []];
  const components = connectedComponents(nodes, edges).sort((a, b) => b.length - a.length || a[0].key.localeCompare(b[0].key));
  for (const component of components) groups[groups[0].length <= groups[1].length ? 0 : 1].push(...component);
  const panels = groups.map((group, index) => {
    const keys = new Set(group.map((node) => node.key));
    const groupEdges = edges.filter((edge) => keys.has(edge.source) && keys.has(edge.target));
    return bestLayoutPanel(country, group, groupEdges, GRAPH_WIDTH, 90, { index: index + 1, total: 2 });
  });
  const failed = panels.find((panel) => !layoutPasses(panel.metrics));
  if (failed) throw new Error(`Akteursnetz layout failed for ${country} part ${failed.part?.index}: ${JSON.stringify(failed.metrics)}`);
  return panels;
}

function edgeFingerprint(edges: ActorEdge[]): string {
  const normalized = [...edges].sort((a, b) => a.key.localeCompare(b.key)).map((edge) => JSON.stringify(edge)).join("\n");
  return createHash("sha256").update(normalized).digest("hex");
}

function renderActorFigures(data: ActorNetworkData, ids: Map<string, string>): string {
  const out = ["\\section{Akteursnetze nach Land}",
    "{\\SemioSans\\fontsize{7.6pt}{9.5pt}\\selectfont Ein Länderblock je Land \\textendash\\ alle streng belegten Verbindungen. Sehr dichte Netze werden in Teilpanels gegliedert. \\SemioGraphLegend{focal=Projekt, attested=neu recherchiert, hypo=Land erschlossen}. Kräftige Linie = projektbezogener Reuse-Beitrag, blasse Linie = geprüfte Organisationsbindung. Zahl = Tabellen-ID.\\\\[2mm]}",
    "\\begin{GraphSpread}"];
  for (const country of COUNTRIES) {
    const nodes = data.nodes.filter((node) => node.country === country);
    const keys = new Set(nodes.map((node) => node.key));
    const edges = data.edges.filter((edge) => keys.has(edge.source) && keys.has(edge.target));
    for (const panel of countryPanels(country, nodes, edges)) {
      const byKey = new Map(panel.nodes.map((node) => [node.key, node]));
      const actors = panel.nodes.filter((node) => node.kind === "actor").length;
      const projects = panel.nodes.length - actors;
      const part = panel.part ? ` \\textperiodcentered\\ Teil ${panel.part.index}/${panel.part.total}` : "";
      out.push(`\\begin{GraphFigure}[title={${COUNTRY_NAMES[country]}${part} \\textperiodcentered\\ ${actors} Organisationen \\textperiodcentered\\ ${projects} Projekte}, width=${panel.width.toFixed(2)}, height=${panel.height.toFixed(2)}]`);
      for (const edge of panel.edges) {
        const source = byKey.get(edge.source)!;
        const target = byKey.get(edge.target)!;
        const kind = edge.kind === "muted" ? "[kind=muted]" : "";
        out.push(`\\SemioGraphEdge${kind}{${source.x!.toFixed(2)},${source.y!.toFixed(2)}}{${target.x!.toFixed(2)},${target.y!.toFixed(2)}}`);
      }
      for (const node of panel.nodes) {
        const options: string[] = [];
        const state = node.kind === "project" ? "focal" : node.state;
        if (state && state !== "default") options.push(`state=${state}`);
        if (node.assetPath) options.push(`image={${node.assetPath}}`);
        out.push(`\\SemioGraphNode${options.length ? `[${options.join(",")}]` : ""}{${node.x!.toFixed(2)},${node.y!.toFixed(2)}}{${ids.get(node.key)}}`);
      }
      out.push("\\end{GraphFigure}", "");
    }
  }
  out.push("\\end{GraphSpread}", "");
  return out.join("\n");
}

function renderActorTables(data: ActorNetworkData, ids: Map<string, string>): string {
  const out = ["\\section{Akteurstabellen nach Land}",
    "{\\SemioSans\\fontsize{7.6pt}{9.5pt}\\selectfont ID = Typ-Buchstabe + laufende Nummer je Land. \\textbf{Q} = Quellennummer. Die sichtbaren Rollengruppen A--P folgen der Klassifikation.\\\\[2mm]}"];
  for (const country of COUNTRIES) {
    const nodes = data.nodes.filter((node) => node.country === country)
      .sort((a, b) => (ids.get(a.key) ?? "").localeCompare(ids.get(b.key) ?? "", undefined, { numeric: true }));
    const urls: string[] = [];
    for (const node of nodes) if (!urls.includes(node.sourceUrl)) urls.push(node.sourceUrl);
    out.push(`\\SemioTableLong[text-size=6.2pt]{${COUNTRY_NAMES[country]}}{0.06,0.29,0.10,0.48,0.07}{ID & Name & Rolle & Relevanz für Wiederverwendung & Q}{%`);
    for (const node of nodes) {
      const role = node.roleGroups.map(tex).join(" \\SemioSlash{} ");
      out.push(`  \\SemioTableRow{${ids.get(node.key)} & ${tex(node.name)} & ${role} & ${tex(node.relevance)} & ${urls.indexOf(node.sourceUrl) + 1}}`);
    }
    out.push("}", "", `\\SemioTableLong[text-size=6pt]{Quellen ${COUNTRY_NAMES[country]}}{0.08,0.92}{Q & URL}{%`);
    for (const [index, url] of urls.entries()) out.push(`  \\SemioTableRow{${index + 1} & \\url{${tex(url)}}}`);
    out.push("}", "\\clearpage", "");
  }
  return out.join("\n");
}

function renderActorPrograms(data: ActorNetworkData): string {
  const urls = [...new Set(data.programs.map((program) => program.evidenceUrl!))];
  const out = ["\\section{Programme}", "\\label{anlage:akteursnetz-programme}", "",
    "Programme sind eigenständige Forschungs- oder Förderprogramme und keine Organisationen.", "",
    "\\SemioTableLong[text-size=7.2pt]{Programme}{0.07,0.23,0.24,0.39,0.07}{Land & Programm & Rolle(n) & Relevanz für Wiederverwendung & Q}{%"];
  for (const program of [...data.programs].sort((a, b) => a.country.localeCompare(b.country) || a.name.localeCompare(b.name)))
    out.push(`  \\SemioTableRow{${tex(program.country)} & ${tex(program.name)} & ${program.roles.map(tex).join(" \\SemioSlash{} ")} & ${tex(program.relevance)} & ${urls.indexOf(program.evidenceUrl!) + 1}}`);
  out.push("}", "", "\\SemioTableLong[text-size=6pt]{Programmquellen}{0.08,0.92}{Q & URL}{%");
  for (const [index, url] of urls.entries()) out.push(`  \\SemioTableRow{${index + 1} & \\url{${tex(url)}}}`);
  out.push("}", "");
  return out.join("\n");
}

function validateActorNetworkRenderSync(data = actorNetworkData()): void {
  const ids = displayIds(data.nodes);
  const expected = new Map([
    [ACTOR_NETWORK_FIGURES, renderActorFigures(data, ids)],
    [ACTOR_NETWORK_TABLES, renderActorTables(data, ids)],
    [ACTOR_NETWORK_PROGRAMS, renderActorPrograms(data)],
  ]);
  for (const [path, content] of expected) if (!existsSync(path) || readFileSync(path, "utf8") !== content) throw new Error(`stale Akteursnetz fragment: ${path}`);
}

function renderActorNetwork(): void {
  const data = actorNetworkData();
  validateActorNetwork(data);
  const edgesBefore = edgeFingerprint(data.edges);
  const ids = displayIds(data.nodes);
  writeFileSync(ACTOR_NETWORK_FIGURES, renderActorFigures(data, ids));
  writeFileSync(ACTOR_NETWORK_TABLES, renderActorTables(data, ids));
  writeFileSync(ACTOR_NETWORK_PROGRAMS, renderActorPrograms(data));
  const projects = data.nodes.filter((node) => node.kind === "project").length;
  const intro = `\\label{anlage:akteursnetz}\n\nDer Anhang dokumentiert das streng belegte Akteursnetz der Bauteil-Wiederverwendung in elf europäischen Ländern: ${data.nodes.length} Knoten, davon ${projects} Referenzprojekte, ${data.edges.length} Verbindungen und ${data.programs.length} getrennt geführte Programme (\\appendixref{anlage:akteursnetz-programme}).\n\nOrganisationen bleiben nur mit einer konkret belegten Reuse-Tätigkeit oder reuse-spezifischen Projektfunktion im Netz. Projekte zeigen tatsächlich geborgene oder eingebaute Bauteile. Partnerlisten, allgemeine Nachhaltigkeitsaussagen, Recycling und nur geplante Wiederverwendung wurden ausgeschlossen. Erhebungs- und Prüfstand: August~2026.\n\n\\input{anhang/akteursnetz-figuren}\n\n\\input{anhang/akteursnetz-tabellen}\n\n\\input{anhang/akteursnetz-programme}\n`;
  writeFileSync(ACTOR_NETWORK_INTRO, intro);
  const edgesAfter = edgeFingerprint(actorNetworkData().edges);
  if (edgesBefore !== edgesAfter) throw new Error(`Akteursnetz edge fingerprint changed during render: ${edgesBefore} -> ${edgesAfter}`);
  console.log(`[DEBUG] Akteursnetz rendered: ${data.nodes.length} nodes, ${data.edges.length} edges, ${data.programs.length} programs, edges ${edgesAfter.slice(0, 12)}`);
}
//#endregion Akteursnetz

//#region Documents
/** 📚 The Zukunft Bau report family of `Entwerfen mit Bestand`, keyed by document name.
 * @see https://www.zukunftbau.de/programme/forschungsfoerderung */
const DOCUMENTS = {
  zwischenbericht: "zwischenbericht/zwischenbericht.tex",
  forschungsbericht: "forschungsbericht/forschungsbericht.tex",
  kompaktbericht: "kompaktbericht/kompaktbericht.tex",
} as const;

function documentTexPath(name: string): string | undefined {
  const relative = DOCUMENTS[name as keyof typeof DOCUMENTS];
  return relative ? join(berichtRoot, relative) : undefined;
}
//#endregion

function resolveTexPath(segments: string[]): string {
  const raw = segments[0];
  if (raw === undefined) throw new Error(`missing document: pass one of ${Object.keys(DOCUMENTS).join(", ")} or a .tex path`);
  const abs = documentTexPath(raw) ?? resolve(raw.endsWith(".tex") ? raw : `${raw}.tex`);
  if (!existsSync(abs)) throw new Error(`missing tex file: ${abs}`);
  return abs;
}

function resolveOutDir(texAbs: string, segments: string[]): string {
  if (segments[1]) return resolve(segments[1]);
  return join(dirname(texAbs), "dist");
}

async function buildDocument(segments: string[]): Promise<void> {
  await fetchPrintFonts();
  const texAbs = resolveTexPath(segments);
  if (texAbs === documentTexPath("forschungsbericht")) {
    const data = actorNetworkData();
    validateActorNetwork(data);
    validateActorNetworkRenderSync(data);
  }
  const outDir = resolveOutDir(texAbs, segments);
  await buildPrintDocument(texAbs, outDir);
}

async function buildDocuments(segments: string[]): Promise<void> {
  if (segments.length > 0) return buildDocument(segments);
  for (const name of Object.keys(DOCUMENTS)) await buildDocument([name]);
}

async function watchDocument(segments: string[]): Promise<void> {
  const texAbs = resolveTexPath(segments);
  const outDir = resolveOutDir(texAbs, segments);
  const roots = [dirname(texAbs), join(berichtRoot, "../../print/tex")];
  const mtimes = new Map<string, number>();
  const rebuild = async () => {
    try {
      await buildDocument([texAbs, outDir]);
    } catch (error) {
      console.error("[DEBUG] mit-bestand/bericht watch rebuild failed:", error);
    }
  };
  await rebuild();
  for (const root of roots) {
    if (!existsSync(root)) continue;
    watch(root, { recursive: true }, (_event, file) => {
      if (!file) return;
      const abs = join(root, file);
      if (abs.includes(".semio-dark") || /-dark\.tex$/i.test(abs)) return;
      if (!/\.(tex|sty|cls|ttf|json)$/i.test(abs)) return;
      try {
        const mtime = statSync(abs).mtimeMs;
        if (mtimes.get(abs) === mtime) return;
        mtimes.set(abs, mtime);
        void rebuild();
      } catch {
        /* ignore */
      }
    });
  }
  console.log(`[DEBUG] mit-bestand/bericht watching ${basename(texAbs)}`);
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildDocuments(segments);
  }
}

class WatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await watchDocument(segments);
  }
}

class LatexScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildDocument(segments);
  }
}

class ActorNetworkScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const command = segments[0] ?? "validate";
    if (command === "validate") {
      const data = actorNetworkData();
      validateActorNetwork(data);
      return validateActorNetworkRenderSync(data);
    }
    if (command === "render") return renderActorNetwork();
    throw new Error(`unknown akteursnetz command: ${command}`);
  }
}

const router = new ScriptRouter(berichtRoot)
  .register("build", BuildScript)
  .register("watch", WatchScript)
  .register("latex", LatexScript)
  .register("akteursnetz", ActorNetworkScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
