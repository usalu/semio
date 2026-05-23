// #region 🧲Header
// 💻 .storybook/vitest.setup.ts
// Specs: Wire Vitest to the root Storybook preview annotations.
// Summary: setProjectAnnotations + beforeAll for Storybook addon-vitest when run from repo root.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { setProjectAnnotations } from "@storybook/react-vite";
import { beforeAll } from "vitest";
import * as projectAnnotations from "./preview";

const project = setProjectAnnotations([projectAnnotations]);

beforeAll(project.beforeAll);
