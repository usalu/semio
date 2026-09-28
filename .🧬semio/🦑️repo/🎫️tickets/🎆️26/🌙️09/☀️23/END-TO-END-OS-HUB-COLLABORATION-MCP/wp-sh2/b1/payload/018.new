/** 📤️ Authoritative direct TypeScript leaf for unlisting one document from this device's local catalog. */

import type { LocalCatalogConfigMutation } from "../🟦️.ts";
import type { LocalCatalog } from "../📥️admit-local-document/🟦️.ts";
import { admitLocalDocument } from "../📥️admit-local-document/🟦️.ts";

//#region 🔖️Mutation
/** 📤️ Removes one document from the local catalog, if listed; its own events stay where they persist. */
export interface RetireLocalDocument {
  readonly documentId: string;
}

/** 🏗️ Wraps a retire payload in the local-catalog dispatch union. */
export function retireLocalDocument(documentId: string): LocalCatalogConfigMutation {
  return { mutation: "retireLocalDocument", documentId };
}

/** 🔺️ Drops the entry for the payload's id; an unlisted id leaves the catalog as it was. */
export function diff(payload: RetireLocalDocument, base: LocalCatalog): LocalCatalog {
  if (!base.documents.some((entry) => entry.documentId === payload.documentId)) return base;
  return { documents: base.documents.filter((entry) => entry.documentId !== payload.documentId) };
}

/** ↩️ Re-admits the prior entry, or emits no step when the id was not listed. */
export function inverse(payload: RetireLocalDocument, base: LocalCatalog): LocalCatalogConfigMutation[] {
  const prior = base.documents.find((entry) => entry.documentId === payload.documentId);
  return prior ? [admitLocalDocument(prior)] : [];
}
//#endregion 🔖️Mutation
