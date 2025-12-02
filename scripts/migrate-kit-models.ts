/**
 * Migration script: kit_metabolism.json.old -> kit_metabolism.json
 *
 * Migrates `representations` from old kit format to proper new schema with files, tags, and models.
 *
 * Schema changes:
 * - OLD: `representations` with url, description, tags (strings), attributes
 * - NEW:
 *   - `files` array: File entities for each unique URL
 *   - `tags` array: Tag entities for each unique tag string
 *   - `models` array: Model entities referencing files and tags by GUID
 *
 * Type identification changes:
 * - OLD: `name` + `variant` (variant can be empty)
 * - NEW: `name` (variant becomes child type name with `parent.guid`)
 */

import { randomUUID } from 'crypto';
import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';

interface OldRepresentation {
    url: string;
    description: string;
    tags: string[];
    attributes: unknown[];
}

interface OldType {
    name: string;
    variant: string;
    representations: OldRepresentation[];
    // ... other fields
}

interface OldKit {
    name: string;
    description: string;
    icon: string;
    image: string;
    preview: string;
    version: string;
    remote: string;
    homepage: string;
    license: string;
    types: OldType[];
    // ... other fields
}

interface File {
    guid: string;
    name: string;
    createdAt: string;
    updatedAt: string;
}

interface Tag {
    guid: string;
    name: string;
}

interface NewModel {
    guid: string;
    file: { guid: string };
    tags?: { guid: string }[];
    description?: string;
    attributes?: unknown[];
}

interface NewType {
    guid: string;
    name: string;
    parent?: { guid: string };
    models?: NewModel[];
    // ... other fields
}

interface NewKit {
    guid: string;
    name?: string;
    description?: string;
    icon?: string;
    image?: string;
    preview?: string;
    version?: string;
    remote?: string;
    homepage?: string;
    license?: string;
    files?: File[];
    tags?: Tag[];
    types: NewType[];
    // ... other fields
}

function main() {
    const basePath = resolve(__dirname, '../assets/semio');
    const oldKitPath = resolve(basePath, 'kit_metabolism.json.old');
    const newKitPath = resolve(basePath, 'kit_metabolism.json');

    console.log('Reading old kit...');
    const oldKit: OldKit = JSON.parse(readFileSync(oldKitPath, 'utf-8'));

    console.log('Reading new kit...');
    const newKit: NewKit = JSON.parse(readFileSync(newKitPath, 'utf-8'));

    // Migrate kit-level metadata
    console.log('\nMigrating kit metadata...');
    if (oldKit.name && !newKit.name) {
        newKit.name = oldKit.name;
        console.log(`  [OK] name: ${oldKit.name}`);
    }
    if (oldKit.description && !newKit.description) {
        newKit.description = oldKit.description;
        console.log(`  [OK] description: ${oldKit.description.substring(0, 50)}...`);
    }
    if (oldKit.icon && !newKit.icon) {
        newKit.icon = oldKit.icon;
        console.log(`  [OK] icon: ${oldKit.icon}`);
    }
    if (oldKit.image && !newKit.image) {
        newKit.image = oldKit.image;
        console.log(`  [OK] image: ${oldKit.image}`);
    }
    if (oldKit.preview && !newKit.preview) {
        newKit.preview = oldKit.preview;
        console.log(`  [OK] preview: ${oldKit.preview}`);
    }
    if (oldKit.remote && !newKit.remote) {
        newKit.remote = oldKit.remote;
        console.log(`  [OK] remote: ${oldKit.remote}`);
    }
    if (oldKit.homepage && !newKit.homepage) {
        newKit.homepage = oldKit.homepage;
        console.log(`  [OK] homepage: ${oldKit.homepage}`);
    }
    if (oldKit.license && !newKit.license) {
        newKit.license = oldKit.license;
        console.log(`  [OK] license: ${oldKit.license}`);
    }

    // Step 1: Collect unique URLs and tags from old representations
    console.log('\n=== Step 1: Collecting unique URLs and tags ===');
    const uniqueUrls = new Set<string>();
    const uniqueTags = new Set<string>();

    for (const oldType of oldKit.types) {
        if (!oldType.representations) continue;
        for (const rep of oldType.representations) {
            uniqueUrls.add(rep.url);
            if (rep.tags) {
                rep.tags.forEach(tag => uniqueTags.add(tag));
            }
        }
    }

    console.log(`Found ${uniqueUrls.size} unique URLs`);
    console.log(`Found ${uniqueTags.size} unique tags`);

    // Step 2: Create File entities
    console.log('\n=== Step 2: Creating File entities ===');
    const urlToFileGuid = new Map<string, string>();
    const files: File[] = [];
    const timestamp = new Date().toISOString();

    for (const url of uniqueUrls) {
        const fileGuid = randomUUID();
        urlToFileGuid.set(url, fileGuid);
        files.push({
            guid: fileGuid,
            name: url,
            createdAt: timestamp,
            updatedAt: timestamp,
        });
    }

    newKit.files = files;
    console.log(`Created ${files.length} File entities`);

    // Step 3: Create Tag entities
    console.log('\n=== Step 3: Creating Tag entities ===');
    const tagNameToGuid = new Map<string, string>();
    const tags: Tag[] = [];

    for (const tagName of uniqueTags) {
        const tagGuid = randomUUID();
        tagNameToGuid.set(tagName, tagGuid);
        tags.push({
            guid: tagGuid,
            name: tagName,
        });
    }

    newKit.tags = tags;
    console.log(`Created ${tags.length} Tag entities`);

    // Build a map of new types by guid for parent lookup
    const newTypesByGuid = new Map<string, NewType>();
    for (const type of newKit.types) {
        newTypesByGuid.set(type.guid, type);
    }

    // Mapping from old type names to new parent names
    const parentNameMapping: Record<string, string> = {
        'Capsule with Balcony': 'Balcony',
        'Ellipsoid Capsule': 'Ellipsoid',
        'Trapezoid Capsule': 'Trapezoid',
    };

    // Build mapping from old (name, variant) to new type
    function findNewType(oldName: string, oldVariant: string): NewType | undefined {
        if (!oldVariant) {
            return newKit.types.find((t) => t.name === oldName && !t.parent);
        } else {
            const mappedParentName = parentNameMapping[oldName] || oldName;
            return newKit.types.find((t) => {
                if (t.name !== oldVariant) return false;
                if (!t.parent) return false;
                const parentType = newTypesByGuid.get(t.parent.guid);
                return parentType && parentType.name === mappedParentName;
            });
        }
    }

    // Step 4: Migrate representations to models
    console.log(`\n=== Step 4: Migrating ${oldKit.types.length} types to models ===`);

    let migratedCount = 0;
    let skippedCount = 0;
    const notFound: string[] = [];

    for (const oldType of oldKit.types) {
        const oldId = oldType.variant ? `${oldType.name}:${oldType.variant}` : oldType.name;

        if (!oldType.representations || oldType.representations.length === 0) {
            console.log(`  [SKIP] ${oldId} - no representations`);
            skippedCount++;
            continue;
        }

        const newType = findNewType(oldType.name, oldType.variant);

        if (!newType) {
            console.log(`  [NOT FOUND] ${oldId}`);
            notFound.push(oldId);
            continue;
        }

        // Convert representations to models with proper file and tag references
        const models: NewModel[] = oldType.representations.map((rep) => {
            const fileGuid = urlToFileGuid.get(rep.url);
            if (!fileGuid) {
                throw new Error(`File GUID not found for URL: ${rep.url}`);
            }

            const model: NewModel = {
                guid: randomUUID(),
                file: { guid: fileGuid },
            };

            // Add tags if present
            if (rep.tags && rep.tags.length > 0) {
                model.tags = rep.tags.map(tagName => {
                    const tagGuid = tagNameToGuid.get(tagName);
                    if (!tagGuid) {
                        throw new Error(`Tag GUID not found for tag: ${tagName}`);
                    }
                    return { guid: tagGuid };
                });
            }

            // Add description if present and non-empty
            if (rep.description && rep.description.trim() !== '') {
                model.description = rep.description;
            }

            // Add attributes if present and non-empty
            if (rep.attributes && rep.attributes.length > 0) {
                model.attributes = rep.attributes;
            }

            return model;
        });

        newType.models = models;
        console.log(`  [OK] ${oldId} -> ${newType.name} (${models.length} models)`);
        migratedCount++;
    }

    console.log(`\n=== Summary ===`);
    console.log(`Files created: ${files.length}`);
    console.log(`Tags created: ${tags.length}`);
    console.log(`Types migrated: ${migratedCount}`);
    console.log(`Types skipped (no representations): ${skippedCount}`);
    console.log(`Types not found: ${notFound.length}`);
    if (notFound.length > 0) {
        console.log(`  Not found types: ${notFound.join(', ')}`);
    }

    // Write updated kit
    console.log(`\nWriting updated kit to ${newKitPath}...`);
    writeFileSync(newKitPath, JSON.stringify(newKit, null, 2));
    console.log('Done!');
}

main();
