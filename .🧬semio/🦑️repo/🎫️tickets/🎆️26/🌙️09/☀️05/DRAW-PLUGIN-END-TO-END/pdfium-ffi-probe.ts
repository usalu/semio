/** 🧮️ Cross-checks Draw writer pixels with the cached, pinned PDFium wheel's rendering API. */
import {dlopen, FFIType as type, ptr} from "bun:ffi";
import {resolve} from "node:path";
import {homedir} from "node:os";

const cache = resolve(homedir(), ".cache/uv/archive-v0");
const wheels = await Array.fromAsync(new Bun.Glob("*/pypdfium2_raw/libpdfium.dylib").scan({cwd: cache}));
if (wheels.length !== 1) throw Error("Expected one cached PDFium wheel");
const library = dlopen(resolve(cache, wheels[0]!), {
    FPDF_InitLibrary: {args: [], returns: type.void},
    FPDF_DestroyLibrary: {args: [], returns: type.void},
    FPDF_LoadMemDocument64: {args: [type.ptr, type.u64, type.ptr], returns: type.ptr},
    FPDF_GetPageCount: {args: [type.ptr], returns: type.i32},
    FPDF_LoadPage: {args: [type.ptr, type.i32], returns: type.ptr},
    FPDF_GetPageWidthF: {args: [type.ptr], returns: type.f32},
    FPDF_GetPageHeightF: {args: [type.ptr], returns: type.f32},
    FPDFBitmap_CreateEx: {args: [type.i32, type.i32, type.i32, type.ptr, type.i32], returns: type.ptr},
    FPDF_RenderPageBitmap: {args: [type.ptr, type.ptr, type.i32, type.i32, type.i32, type.i32, type.i32, type.i32], returns: type.void},
    FPDFBitmap_Destroy: {args: [type.ptr], returns: type.void},
    FPDF_ClosePage: {args: [type.ptr], returns: type.void},
    FPDF_CloseDocument: {args: [type.ptr], returns: type.void},
} as const);
const pdf = library.symbols;
pdf.FPDF_InitLibrary();
try {
    for (let index = 0; index < 25; index++) {
        const bytes = new Uint8Array(await Bun.file(new URL("./🗑️generated/pdfium-input-" + index + ".pdf", import.meta.url)).arrayBuffer());
        const document = pdf.FPDF_LoadMemDocument64(ptr(bytes), BigInt(bytes.byteLength), null);
        if (!document) throw Error("Missing document " + index);
        try {
            if (pdf.FPDF_GetPageCount(document) !== 1) throw Error("Unexpected page count " + index);
            const page = pdf.FPDF_LoadPage(document, 0);
            if (!page) throw Error("Missing page " + index);
            try {
                if (pdf.FPDF_GetPageWidthF(page) !== 2 || pdf.FPDF_GetPageHeightF(page) !== 1) throw Error("Unexpected extent " + index);
                const surface = new Uint8Array(512 * 256 * 4).fill(255);
                const bitmap = pdf.FPDFBitmap_CreateEx(512, 256, 4, ptr(surface), 512 * 4);
                if (!bitmap) throw Error("Missing bitmap " + index);
                try {
                    pdf.FPDF_RenderPageBitmap(bitmap, page, 0, 0, 512, 256, 0, 0);
                    const samples = [128, 384].flatMap(x => {
                        const offset = (128 * 512 + x) * 4;
                        return [surface[offset + 2], surface[offset + 1], surface[offset], surface[offset + 3]];
                    });
                    console.log(index, ...samples);
                } finally { pdf.FPDFBitmap_Destroy(bitmap); }
            } finally { pdf.FPDF_ClosePage(page); }
        } finally { pdf.FPDF_CloseDocument(document); }
    }
} finally {
    pdf.FPDF_DestroyLibrary();
    library.close();
}
