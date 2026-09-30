import { expect, test } from "bun:test";
import extensionEvaluateFaultFixture from "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/💥️extension-evaluate-fault.json";
  // 💥️ The preview status a refused `evaluate` publishes, over the SAME fixture the guest law
  // `an_evaluate_fault_outranks_the_addressing_miss_and_a_contribution_install_clears_it` drives
  // (`🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`). The host never authors this object — it
  // renders it — so what this pins is the shape and the both-languages rule the surface depends on.
test("pins the localized evaluate-fault the preview status publishes", () => {
    const fixture = extensionEvaluateFaultFixture;
    expect(fixture.code).toBe("flow.extension-evaluate-failed");
    expect(new Set(fixture.phases).size).toBe(fixture.phases.length);
    for (const row of fixture.answers) {
      expect(fixture.phases).toContain(row.phase);
      expect(row.publishesFault).toBe(row.phase === "faulted");
      expect(row.publishesFault).toBe(row.ok === false && row.faultCode.length > 0);
    }
    const languages = Object.keys(fixture.labels);
    expect(languages).toEqual(["en", "de"]);
    expect(new Set(Object.values(fixture.labels)).size).toBe(languages.length);
    for (const text of Object.values(fixture.labels)) expect(text).toContain(fixture.extensionId);
    expect(fixture.clearedByContributionInstall).toBe(true);
  });
