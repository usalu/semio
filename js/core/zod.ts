// #region Header

// zod.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
//zod.ts
//2025 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Lesser General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Lesser General Public License for more details.

//You should have received a copy of the GNU Lesser General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

// https://transform.tools/json-schema-to-zod

import { z } from 'zod'

const Kit = z
  .object({
    uri: z.string().max(2048).describe('🆔 The uri of the kit.'),
    name: z.string().max(64).describe('📛 The name of the kit.'),
    description: z.string().max(2560).describe('💬 The optional human-readable description of the kit.').optional(),
    icon: z
      .string()
      .max(1024)
      .describe(
        '🪙 The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. kit.'
      )
      .optional(),
    image: z
      .string()
      .max(1024)
      .describe(
        '🖼️ The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.'
      )
      .optional(),
    preview: z
      .string()
      .max(1024)
      .describe(
        '🔮 The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB.'
      )
      .optional(),
    version: z
      .string()
      .max(64)
      .describe('🔀 The optional version of the kit. No version means the latest version.')
      .optional(),
    remote: z
      .string()
      .max(1024)
      .describe('☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely.')
      .optional(),
    homepage: z.string().max(1024).describe('🏠 The optional url of the homepage of the kit.').optional(),
    license: z.string().max(1024).describe('⚖️ The optional license [ spdx id | url ] of the kit.').optional(),
    created: z.string().describe('🕒 The creation date of the kit.').optional(),
    updated: z.string().describe('🕒 The last update date of the kit.').optional(),
    types: z.array(z.any()).describe('🧩 The types of the kit.').optional(),
    designs: z.array(z.any()).describe('🏙️ The designs of the kit.').optional(),
    qualities: z.array(z.any()).describe('📏 The qualities of the kit.').optional()
  })
  .describe('🗃️ A kit is a collection of types and designs.')

type Kit = z.infer<typeof Kit>
export type { Kit }
