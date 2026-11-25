import { existsSync } from "node:fs";
import { chromium } from "playwright";

const executablePath = process.env.SEMIO_NATIVE_TEST_CHROMIUM;
if (!executablePath || !existsSync(executablePath)) throw new Error("Set SEMIO_NATIVE_TEST_CHROMIUM to the installed Chromium executable");

const launch = chromium.launch.bind(chromium);
chromium.launch = options => launch({ ...options, executablePath });
chromium.executablePath = () => executablePath;
