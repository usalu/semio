import { describe, expect, it } from "vitest";
import { auditAllPlaygrounds } from "./audit-playground-completeness.ts";

describe("playground window/mode completeness", () => {
	it(
		"every playground mode has footer tools and every window has measures and engagement",
		async () => {
			const failures = await auditAllPlaygrounds();
			expect(failures).toEqual([]);
		},
		120_000,
	);
});
