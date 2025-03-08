import type { GatsbyConfig } from "gatsby";

const config: GatsbyConfig = {
    // siteMetadata: {
    //     siteUrl: `https://www.yourdomain.tld`,
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
    plugins: [`gatsby-plugin-mdx`, `gatsby-plugin-postcss`, `gatsby-plugin-sass`]
};

export default config;