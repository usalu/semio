import type { ForgeConfig } from '@electron-forge/shared-types';
// import { MakerDeb } from '@electron-forge/maker-deb';
// import { MakerSquirrel } from '@electron-forge/maker-squirrel';
// import { MakerZIP } from '@electron-forge/maker-zip';


const config: ForgeConfig = {
    plugins: [
        {
            name: '@electron-forge/plugin-vite',
            config: {
                // `build` can specify multiple entry builds, which can be
                // Main process, Preload scripts, Worker process, etc.
                build: [
                    {
                        // `entry` is an alias for `build.lib.entry`
                        // in the corresponding file of `config`.
                        entry: 'main.ts',
                        // config: '../core/vite.config.mts'
                        // config: 'vite.main.config.mts'
                        config: 'vite.main.config.mts'
                    },
                    {
                        entry: 'preload.ts',
                        // config: '../core/vite.config.mts'
                        config: 'vite.preload.config.mts'
                    }
                ],
                renderer: [
                    {
                        name: 'main_window',
                        // config: '../core/vite.config.mts'
                        config: 'vite.renderer.config.mts'
                    }
                ]
            }
        }
    ],
    // packagerConfig: {
    //     osxSign: {},
    //     osxNotarize: {
    //         tool: 'notarytool',
    //         appleId: process.env.APPLE_ID,
    //         appleIdPassword: process.env.APPLE_PASSWORD,
    //         teamId: process.env.APPLE_TEAM_ID
    //     }
    // },
    // makers: [
    // new MakerSquirrel({
    //     authors: 'Electron contributors'
    // }, ['win32']),
    // new MakerZIP({}, ['darwin']),
    // new MakerDeb({}, ['linux']),
    // new MakerRpm({}, ['linux']),
    // ],
    publishers: [
        {
            name: '@electron-forge/publisher-github',
            config: {
                repository: {
                    owner: 'usalu',
                    name: 'semio'
                },
                prerelease: false,
                draft: true
            }
        }
    ]
}

export default config;