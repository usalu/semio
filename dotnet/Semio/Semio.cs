using System;
using System.Collections.Generic;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.Newtonsoft;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using Semio.Properties;

namespace Semio
{
    public static class Serializer
    {
        public static string Serialize(this object obj)
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(
                obj,Formatting.Indented, new JsonSerializerSettings
            {
                ContractResolver = new CamelCasePropertyNamesContractResolver()
            });
        }
    }

    public static class Deserializer
    {
        public static T Deserialize<T>(this string json)
        {
            return Newtonsoft.Json.JsonConvert.DeserializeObject<T>(
                               json, new JsonSerializerSettings
                               {
                ContractResolver = new CamelCasePropertyNamesContractResolver()
            });
        }
    }
    public class Representation
    {
        public string Url { get; set; }
        public string Lod { get; set; }
        public List<string> Tags { get; set; }

        public override string ToString() => $"Representation(Url: {Url})";

    }
    public class Specifier
    {
        public string Context { get; set; }
        public string Group { get; set; }

        public override string ToString() => $"Specifier(Context: {Context})";
    }

    public class Point
    {
        public float X { get; set; }
        public float Y { get; set; }
        public float Z { get; set; }

        public override string ToString() => $"Point(X: {X}, Y: {Y}, Z: {Z})";

    }

    public class Vector
    {
        public float X { get; set; }
        public float Y { get; set; }
        public float Z { get; set; }

        public override string ToString() => $"Vector(X: {X}, Y: {Y}, Z: {Z})";
    }

    public class Plane
    {
        public Point Origin { get; set; }
        public Vector XAxis { get; set; }
        public Vector YAxis { get; set; }

        public override string ToString() => $"Plane(Origin: {Origin}, XAxis: {XAxis}, YAxis: {YAxis})";
    }

    public class Port
    {
        public Plane Plane { get; set; }
        public List<Specifier> Specifiers { get; set; }

        public override string ToString() => $"Port({GetHashCode()})";

    }

    public class PortId
    {
        public List<Specifier> Specifiers { get; set; }
        public override string ToString() => $"PortId({GetHashCode()})";

    }

    public class Quality
    {
        public string Name { get; set; }
        public string Value { get; set; }
        public string Unit { get; set; }

        public override string ToString() => $"Quality(Name: {Name})";
    }

    public class Type
    {
        public string Name { get; set; }
        public string Explanation { get; set; }
        public string Icon { get; set; }
        public List<Representation> Representations { get; set; }
        public List<Port> Ports { get; set; }
        public List<Quality> Qualities { get; set; }

        public override string ToString() => $"Type(Name: {Name})";

    }

    public class TypeId
    {
        public string Name { get; set; }
        public List<Quality> Qualities { get; set; }
    }

    public class Piece
    {
        public string Id { get; set; }
        public TypeId Type { get; set; }
    }


    public class TypePieceSide
    {
        public PortId Port { get; set; }
    }

    public class PieceSide
    {
        public string Id { get; set; }
        public TypePieceSide Type { get; set; }
    }

    public class Side
    {
        public PieceSide Piece { get; set; }
    }


    public class Attraction
    {
        public Side Attracting { get; set; }
        public Side Attracted { get; set; }
    }

    public class Formation
    {
        public string Name { get; set; }
        public string Explanation { get; set; }
        public string Icon { get; set; }
        public List<Piece> Pieces { get; set; }
        public List<Attraction> Attractions { get; set; }
        public List<Quality> Qualities { get; set; }
    }

    public class Kit
    {
        public string Name { get; set; }
        public string Explanation { get; set; }
        public string Icon { get; set; }
        public string Url { get; set; }
        public List<Type> Types { get; set; }
        public List<Formation> Formations { get; set; }
    }

    public class LoadLocalKitResponse
    {
        public Kit Kit { get; set; }
        public string Error { get; set; }
    }

    public class LoadLocalKitResponseContainer
    {
        public LoadLocalKitResponse LoadLocalKit { get; set; }
    }


    public class Api : ICloneable
    {
        public Api()
        {
            Endpoint = "http://127.0.0.1:5000/graphql";
            Token = "";
            Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
        }

        public Api(string endpoint, string token)
        {
            Endpoint = endpoint;
            Token = token;
            Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
            Client.HttpClient.DefaultRequestHeaders.Add("Authorization", $"Bearer {Token}");
        }

        public GraphQLHttpClient Client { get; set; }
        public string Endpoint { get; set; }
        public string Token { get; set; }

        public object Clone()
        {
            return new Api(Endpoint, Token);
        }

        public override string ToString()
        {
            return $"Api(Endpoint: {Endpoint}, Token: {Token})";
        }

        public Kit LoadLocalKit(string directory)
        {
            var query = new GraphQLRequest
            {
                Query = Resources.loadLocalKit,
                OperationName = "LoadLocalKit",
                Variables = new { directory }
            };
            var response = Client.SendQueryAsync<LoadLocalKitResponseContainer>(query).Result;
            return response.Data.LoadLocalKit.Kit;
        }

    }
}