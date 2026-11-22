/** 🪪 Captures a scoped first archive through the local dev folder authority. */
import { BACKBONE_ENDPOINT_PATH } from "@semio-tech/framework-os";
import type { FirstArchiveCapturePortV1 } from "@semio-tech/framework-renderer-react";

const route = `${BACKBONE_ENDPOINT_PATH}/first-archive`;
const hex64 = /^[a-f0-9]{64}$/u;
const decimal = /^(?:0|[1-9][0-9]*)$/u;
const maximumU64 = 0xffff_ffff_ffff_ffffn;
const pageBytes = 4096;
const requestTimeoutMs = 30_000;

function toHex(bytes: Uint8Array): string {
  return Array.from(bytes, byte => byte.toString(16).padStart(2,"0")).join("");
}
function fromHex(value: unknown): Uint8Array<ArrayBuffer> {
  if (typeof value !== "string" || !hex64.test(value)) throw new Error("first-archive.invalid-digest");
  return Uint8Array.from({length:32}, (_unused,index) => Number.parseInt(value.slice(index*2,index*2+2),16));
}
function generation(value: unknown): bigint {
  if (typeof value !== "string" || value.length > 20 || !decimal.test(value)) throw new Error("first-archive.invalid-generation");
  const parsed = BigInt(value);
  if (parsed > maximumU64) throw new Error("first-archive.invalid-generation");
  return parsed;
}
function record(value: unknown): Record<string,unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("first-archive.invalid-receipt");
  return value as Record<string,unknown>;
}

/** 🔏 Binds every request to its server-minted capability and immutable storage frontier. */
export function createDevFirstArchiveCaptureV1(request: typeof fetch = fetch): FirstArchiveCapturePortV1 {
  let ownerEpoch = 0;
  let clientInstanceId: string | undefined;
  const send = async (action: string, init: RequestInit, signal?: AbortSignal, retired = false) => {
    const timeout = AbortSignal.timeout(requestTimeoutMs);
    const response = await request(`${route}/${action}`, { ...init, signal: signal === undefined ? timeout : AbortSignal.any([signal,timeout]) });
    if (!response.ok && !(retired && response.status === 409)) throw new Error(`first-archive.${action}-refused:${response.status}`);
    return response;
  };
  return async ({frontier,source,...scope},signal) => {
    signal.throwIfAborted();
    if (ownerEpoch >= Number.MAX_SAFE_INTEGER) throw new Error("first-archive.owner-epoch-exhausted");
    const reserved = record(await (await send("reserve", {
      method:"POST", headers:{"content-type":"application/json"},
      body:JSON.stringify({
        ...scope, clientInstanceId:clientInstanceId ??= crypto.randomUUID(), ownerEpoch:++ownerEpoch,
        source:{...source,sequence:source.sequence.toString()},
        frontier:{...frontier,generation:frontier.generation.toString(),revision:toHex(frontier.revision)},
      }),
    },signal)).json());
    const {capability,epoch} = reserved;
    if (typeof capability !== "string" || !hex64.test(capability) || typeof epoch !== "number" || !Number.isSafeInteger(epoch) || epoch < 1) {
      throw new Error("first-archive.invalid-reservation");
    }
    const authority = {authorization:`SemioFirstArchive ${capability}`,"x-semio-folder-epoch":String(epoch)};
    const body = { ...authority, "content-type":"application/octet-stream" };
    let retirement: Promise<void> | undefined;
    return {
      async write(offset, page) {
        if (!Number.isSafeInteger(offset) || offset < 0 || offset % pageBytes !== 0 || page.length !== Math.min(pageBytes,frontier.extent-offset) || page.length === 0) {
          throw new Error("first-archive.page-extent");
        }
        const written = record(await (await send(`page?offset=${offset}`, {method:"PUT",headers:body,body:page.slice()},signal)).json());
        if (written.offset !== offset || written.length !== page.length) throw new Error("first-archive.foreign-page");
        return {offset,length:page.length};
      },
      async publish(digest, publishSignal) {
        if (digest.length !== 32) throw new Error("first-archive.invalid-digest");
        const saved = record(await (await send("publish", {method:"POST",headers:body,body:digest.slice()},publishSignal)).json());
        const sealed = record(saved.frontier);
        if (sealed.documentId !== frontier.documentId || saved.extent !== frontier.extent || sealed.extent !== frontier.extent) {
          throw new Error("first-archive.foreign-frontier");
        }
        return {
          extent:frontier.extent,digest:fromHex(saved.digest),
          frontier:{documentId:sealed.documentId,generation:generation(sealed.generation),revision:fromHex(sealed.revision),extent:frontier.extent},
        };
      },
      retire() {
        retirement ??= send("retire",{method:"POST",headers:authority},undefined,true).then(() => undefined, error => {
          retirement=undefined;
          throw error;
        });
        return retirement;
      },
    };
  };
}
