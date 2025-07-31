// #region Header

// utils.ts

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
import { clsx, type ClassValue } from "clsx"
import JSZip from "jszip"
import { twMerge } from "tailwind-merge"

import { default as adjectives } from "@semio/assets/lists/adjectives.json"
import { default as animals } from "@semio/assets/lists/animals.json"


export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

export const extractFilesAndCreateUrls = async (url: string): Promise<Map<string, string>> => {
    const fileUrls: Map<string, string> = new Map()

    try {
        const zipData = await fetch(url).then(res => res.arrayBuffer())
        const zip = await JSZip.loadAsync(zipData)

        for (const fileEntry of Object.values(zip.files)) {
            if (!fileEntry.dir) {
                const fileData = await fileEntry.async("uint8array")
                const mimeType = fileEntry.name.split(".").pop() || ""
                const blob = new Blob([fileData], { type: mimeType })
                const blobUrl = URL.createObjectURL(blob)
                fileUrls.set(fileEntry.name, blobUrl)
            }
        }
    } catch (error) {
        console.error(`Failed to extract files from ${url}:`, error)
        throw new Error(`Failed to extract files from ${url}`)
    }

    return fileUrls
}


class SeededRandom {
    private seed: number
    constructor(seed: number) {
        this.seed = seed % 2147483647
        if (this.seed <= 0) this.seed += 2147483646
    }
    next = (): number => (this.seed = (this.seed * 16807) % 2147483647)
    nextFloat = (): number => (this.next() - 1) / 2147483646
    nextInt = (max: number): number => Math.floor(this.nextFloat() * max)
}

export class Generator {
    public static randomId(seed: number = Math.floor(Math.random() * 1000000)): string {
        const random = new SeededRandom(seed)
        let adjective = adjectives[random.nextInt(adjectives.length)]
        let animal = animals[random.nextInt(animals.length)]
        adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1)
        animal = animal.charAt(0).toUpperCase() + animal.slice(1)
        return `${adjective}${animal}${random.nextInt(1000)}`
    }
    public static randomName(seed: number = Math.floor(Math.random() * 1000000)): string {
        const random = new SeededRandom(seed)
        let animal = animals[random.nextInt(animals.length)]
        animal = animal.charAt(0).toUpperCase() + animal.slice(1)
        return `${animal}`
    }
    public static randomVariant(seed: number = Math.floor(Math.random() * 1000000)): string {
        const random = new SeededRandom(seed)
        let adjective = adjectives[random.nextInt(adjectives.length)]
        adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1)
        return `${adjective}`
    }
}
