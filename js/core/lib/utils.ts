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
import { default as adjectives } from "@semio/assets/lists/adjectives.json"
import { default as animals } from "@semio/assets/lists/animals.json"
import { clsx, type ClassValue } from "clsx"
import JSZip from 'jszip'
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

class SeededRandom {
    private seed: number

    constructor(seed: number) {
        this.seed = seed % 2147483647
        if (this.seed <= 0) this.seed += 2147483646
    }

    // Returns a pseudo-random number between 1 and 2^31 - 2
    next(): number {
        return (this.seed = (this.seed * 16807) % 2147483647)
    }

    // Returns a pseudo-random number between 0 (inclusive) and 1 (exclusive)
    nextFloat(): number {
        return (this.next() - 1) / 2147483646
    }

    // Returns a pseudo-random number between 0 (inclusive) and max (exclusive)
    nextInt(max: number): number {
        return Math.floor(this.nextFloat() * max)
    }
}

export class Generator {
    public static randomId(seed?: number | undefined): string {
        if (seed === undefined) {
            seed = Math.floor(Math.random() * 1000000)
        }
        const random = new SeededRandom(seed)

        let adjective = adjectives[random.nextInt(adjectives.length)]
        let animal = animals[random.nextInt(animals.length)]
        const number = random.nextInt(1000)

        adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1)
        animal = animal.charAt(0).toUpperCase() + animal.slice(1)

        return `${adjective}${animal}${number}`
    }
}

export const jaccard = (a: string[] | undefined, b: string[] | undefined) => {
    if ((a === undefined && b === undefined) || (a?.length === 0 && b?.length === 0)) return 1
    const setA = new Set(a)
    const setB = new Set(b)
    const intersection = [...setA].filter((x) => setB.has(x)).length
    const union = setA.size + setB.size - intersection
    if (union === 0) return 0
    return intersection / union
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

/**
 * Sets a quality in a qualities array. If a quality with the same name exists, it is overwritten.
 * @param qualities - The array of qualities to modify
 * @param name - The name of the quality to set
 * @param value - The value of the quality
 * @param unit - Optional unit of the quality
 * @param definition - Optional definition of the quality
 * @returns The updated qualities array
 */
export const setQuality = (
    qualities: Array<{ name: string, value?: string, unit?: string, definition?: string }> | undefined,
    name: string,
    value?: string,
    unit?: string,
    definition?: string
): Array<{ name: string, value?: string, unit?: string, definition?: string }> => {
    const qualitiesArray = qualities || []
    const existingIndex = qualitiesArray.findIndex(q => q.name === name)

    const newQuality = { name, value, unit, definition }

    if (existingIndex >= 0) {
        // Replace existing quality
        qualitiesArray[existingIndex] = newQuality
    } else {
        // Add new quality
        qualitiesArray.push(newQuality)
    }

    return qualitiesArray
}

/**
 * Sets multiple qualities in a qualities array. For each quality, if one with the same name exists, it is overwritten.
 * @param qualities - The array of qualities to modify
 * @param newQualities - Array of qualities to set
 * @returns The updated qualities array
 */
export const setQualities = (
    qualities: Array<{ name: string, value?: string, unit?: string, definition?: string }> | undefined,
    newQualities: Array<{ name: string, value?: string, unit?: string, definition?: string }>
): Array<{ name: string, value?: string, unit?: string, definition?: string }> => {
    return newQualities.reduce((acc, quality) =>
        setQuality(acc, quality.name, quality.value, quality.unit, quality.definition),
        qualities || []
    )
}

/**
 * Normalizes a value by converting undefined to empty string
 * @param val - The value to normalize
 * @returns Empty string if value is undefined, otherwise the original value
 */
const normalize = (val: string | undefined): string => (val === undefined ? '' : val)

/**
 * Finds a design in a kit by its design ID
 * @param kit - The kit containing the designs
 * @param designId - The design identifier with name, variant, and view
 * @returns The matching design or undefined if not found
 */
export const findDesign = <T extends { name: string, variant?: string, view?: string }>(
    kit: { designs?: T[] },
    designId: { name: string, variant?: string, view?: string }
): T | undefined => {
    return kit.designs?.find(
        (d) =>
            d.name === designId.name &&
            normalize(d.variant) === normalize(designId.variant) &&
            normalize(d.view) === normalize(designId.view)
    )
}