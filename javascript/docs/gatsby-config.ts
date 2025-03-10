import type { GatsbyConfig } from "gatsby";

const config: GatsbyConfig = {
    // siteMetadata: {
    //     siteUrl: `https://docs.semio-tech.com`,
    // },
    // graphqlTypegen: true,
    // plugins: [
    //     {
    //         resolve: `gatsby-plugin-compile-es6-packages`,
    //         options: {
    //             modules: [`@repo/ui`],
    //         },
    //     },
    // ],
    plugins: [
        `gatsby-plugin-mdx`,
        `gatsby-plugin-postcss`,
        `gatsby-plugin-sass`,
        // {
        //     resolve: `gatsby-source-filesystem`,
        //     options: {
        //         name: `root`,
        //         path: `${__dirname}`,
        //     },
        // },
    ],
};

export default config;