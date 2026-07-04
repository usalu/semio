export type WriterDocument = {
	readonly schema: string;
	readonly id: string;
	readonly languageId: string;
	readonly text: string;
};

export function createWriterDocument(input: { readonly id: string; readonly languageId: string; readonly text: string }): WriterDocument {
	return { schema: "writer.document", id: input.id, languageId: input.languageId, text: input.text };
}
