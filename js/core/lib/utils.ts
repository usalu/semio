import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { default as animals } from "@semio/assets/lists/animals.json"
import { default as adjectives } from "@semio/assets/lists/adjectives.json"
import JSZip from 'jszip'

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