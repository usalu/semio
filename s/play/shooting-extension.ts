/** @emoji 🎯 S-play shooting VCS handler with mesh input binding. */

import { createTypedAppVcsHandler } from "@semio-tech/s-core";
import { baselineSingleAppPlatformDefinition, type PlatformDefinition } from "@semio-tech/framework-platform-core";
import { DEFAULT_SHOOTING_FIXTURE, type ShootingFixture } from "@semio-tech/shooting-react";

export function buildShootingProgramDefinition(): PlatformDefinition {
	return baselineSingleAppPlatformDefinition("shooting", "Shooting", "shooting", "Shooting", "shooting-play");
}

export function createSPlayShootingAppVcsHandler() {
	return createTypedAppVcsHandler<ShootingFixture, { readonly op: "noop" }>(
		"shooting.scene",
		"shooting.fixture",
		() => DEFAULT_SHOOTING_FIXTURE,
		(fixture) => fixture,
		undefined,
		{
			applyInputBindings: (fixture, inputBindings) => {
				const mesh = inputBindings.mesh as { readonly url?: string } | undefined;
				if (!mesh?.url) return fixture;
				const activeId = fixture.activeAssetId ?? fixture.assets[0]?.id;
				if (!activeId) return fixture;
				return {
					...fixture,
					assets: fixture.assets.map((asset) => (asset.id === activeId ? { ...asset, url: mesh.url! } : asset)),
				};
			},
		},
	);
}
