module.exports = {
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
    //     {
    //         name: '@electron-forge/maker-squirrel',
    //         config: {
    //             certificateFile: './cert.pfx',
    //             certificatePassword: process.env.CERTIFICATE_PASSWORD
    //         }
    //     }
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