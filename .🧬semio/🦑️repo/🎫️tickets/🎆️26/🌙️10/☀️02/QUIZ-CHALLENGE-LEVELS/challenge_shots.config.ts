/** 📸️ Playwright configuration of the challenge screenshots: the spec beside this file against a private stack whose site
 * origin is `PLAYWRIGHT_BASE_URL` (the `architektur-und-technologie-quizze-beside` launch row: site 6063, proctor 8793), a
 * desktop and a phone project. Run from the site package: `bunx playwright test --config <this file>`.
 * @see ./challenge_shots.spec.ts */
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6063";

export default {
  testDir: ".",
  testMatch: ["challenge_shots.spec.ts"],
  fullyParallel: true,
  workers: 2,
  retries: 0,
  timeout: 600_000,
  expect: { timeout: 20_000 },
  reporter: [["list"]],
  use: { baseURL, actionTimeout: 20_000, navigationTimeout: 60_000 },
  projects: [
    { name: "desktop", use: { viewport: { width: 1440, height: 900 } } },
    { name: "phone", use: { viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true } },
  ],
};
