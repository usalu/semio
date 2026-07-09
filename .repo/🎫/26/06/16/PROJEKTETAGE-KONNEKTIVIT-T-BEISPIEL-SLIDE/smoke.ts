#!/usr/bin/env bun
/** @emoji 🧪 Smoke-check Konnektivität Beispiel slide deck wiring. */
import { collectPresentationSlides, loadPresentationFromSlideGlob } from "@semio-tech/framework-presentation-core";
import { presentationMeta } from "./spec.ts";

const slideModules = import.meta.glob<{ default: unknown }>("./slide/**/*.ts", { eager: true });
const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
const systematik = deck.chapters.find((chapter) => chapter.name === "Bauteilportal")?.sequences.find((sequence) => sequence.name === "Systematik");
const konnektivität = systematik?.thoughts.find((thought) => thought.name === "Konnektivität");
const beispiel = konnektivität?.slides.find((slide) => slide.arrangement.name === "Beispiel");

if (!beispiel) {
  throw new Error("Konnektivität / Beispiel slide missing");
}
const [figure, table] = beispiel.arrangement.dispositions;
if (figure.participantId !== "konnektivität-beispiel-3d") {
  throw new Error(`unexpected figure participant: ${figure.participantId}`);
}
if (table.participantId !== "konnektivität-beispiel-tabelle") {
  throw new Error(`unexpected table participant: ${table.participantId}`);
}
if ((figure.position?.x ?? 0) + (figure.position?.width ?? 0) > 0.5) {
  throw new Error("figure is not confined to left half");
}
if ((table.position?.x ?? 0) < 0.5) {
  throw new Error("table is not on right half");
}

const bookmark = collectPresentationSlides(deck).find((slide) => slide.thought === "Konnektivität" && slide.slide === "Beispiel");
console.log("[DEBUG] Konnektivität Beispiel bookmark:", bookmark);
console.log("ok");
