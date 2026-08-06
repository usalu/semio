#!/usr/bin/env bun
import { spawn } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";
import {
  loadFrameworkOsPlaygroundCatalog,
  frameworkOsPlaygroundDefaultPort,
} from "../../../../../../../../../../../../../../..";
